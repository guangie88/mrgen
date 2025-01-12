mod conf;

use anyhow::Result;
use clap::Parser;
use conf::Conf;
use git2::Repository;
use itertools::Itertools;
use semver::Version;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Parser)]
#[command(version, about, about = "Command args to pass into mrgen")]
struct Args {
    #[arg(
        short = 'c',
        help = "Overrides mrgen config file path",
        default_value = ".mrgen.yaml"
    )]
    conf: PathBuf,

    #[structopt(short = 'r', help = "Overrides git repo path", default_value = ".")]
    repo: PathBuf,

    #[structopt(short = 'p', help = "Hints to have v prefix")]
    has_prefix: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let conf: Conf = {
        let f = File::open(&args.conf)?;
        serde_yaml::from_reader(f)?
    };

    let repo = Repository::open(&args.repo)?;
    let tags = repo.tag_names(None)?;

    // Iterate over the tags and print them
    let semver_tags: Vec<_> = tags
        .iter()
        .filter_map(|t| {
            // Allow v or V prefix for semver
            let t = t?;
            let t = if t.starts_with("v") || t.starts_with("V") {
                &t[1..]
            } else {
                t
            };
            Version::parse(t).ok()
        })
        .sorted()
        .rev()
        .collect();

    for t in semver_tags.iter() {
        println!("{}", t);
    }

    let most_recent_tag = semver_tags
        .first()
        .map(|t| t.to_string())
        .unwrap_or_else(|| "".to_owned());

    println!("Most recent tag: {most_recent_tag}");

    let mut revwalk = repo.revwalk()?;
    let prefix = if args.has_prefix { "v" } else { "" };
    // revwalk.push_head()?;
    revwalk.push_range(&format!("{prefix}{most_recent_tag}..HEAD"))?;

    // Iterate over the commits
    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;
        println!(
            "Commit: {}\nAuthor: {}\nMessage: {}\n",
            commit.id(),
            commit.author(),
            commit.message().unwrap_or("No message")
        );
    }

    Ok(())
}
