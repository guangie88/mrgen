use anyhow::Result;
use git2::Repository;
use itertools::Itertools;
use semver::Version;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Args", about = "Command args to pass into mrgen")]
struct Opt {
    #[structopt(
        parse(from_os_str),
        short = "c",
        help = "Overrides mrgen config file path",
        default_value = ".mrgen.yaml"
    )]
    conf: PathBuf,

    #[structopt(
        parse(from_os_str),
        short = "r",
        help = "Overrides git repo path",
        default_value = "."
    )]
    repo: PathBuf,

    #[structopt(short = "p", help = "Hints to have v prefix")]
    has_prefix: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let repo = Repository::open(&opt.repo)?;
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
    let prefix = if opt.has_prefix { "v" } else { "" };
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
