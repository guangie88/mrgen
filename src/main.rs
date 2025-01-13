mod conf;

use anyhow::{Error, Result};
use clap::Parser;
use conf::{
    Conf,
    FilesInclusionMode::{FilesExcludeAllFirst, FilesIncludeAllFirst},
};
use git2::{
    DiffFormat::{self},
    Repository,
};
use itertools::Itertools;
use semver::Version;
use std::{
    fs::File,
    path::PathBuf,
    str::{from_utf8, Utf8Error},
};

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

    #[structopt(short = 'w', help = "Specify workspace path", default_value = ".")]
    workspace: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let conf: Conf = {
        let f = File::open(&args.conf)?;
        serde_yaml::from_reader(f)?
    };

    let repo = Repository::open(&args.repo)?;
    let tags = repo.tag_names(None)?;

    let workspace = conf
        .workspaces
        .iter()
        .find(|w| w.path == args.workspace)
        .ok_or_else(|| {
            Error::msg(format!(
                "{} does not match any workspace path, aborting.",
                args.workspace.to_string_lossy()
            ))
        })?;

    let tag_prefix = match &workspace.general.tag_prefix {
        Some(p) => p,
        None => "",
    };

    // Iterate over the tags and print them
    let matching_tags: Vec<_> = tags
        .iter()
        .filter_map(|t| match t {
            Some(t) if t.starts_with(tag_prefix) => {
                let t = t.trim_start_matches(tag_prefix);
                Version::parse(t).ok()
            }
            _ => None,
        })
        .sorted()
        .rev()
        .collect();

    for t in matching_tags.iter() {
        println!("{}", t);
    }

    let most_recent_tag = matching_tags
        .first()
        .map(|t| t.to_string())
        .unwrap_or_else(|| "".to_owned());

    println!("Most recent tag: {most_recent_tag}\n");

    let mut revwalk = repo.revwalk()?;
    revwalk.push_range(&format!("{tag_prefix}{most_recent_tag}..HEAD"))?;

    // Iterate over the commits
    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;

        let new_tree = commit.tree()?;
        let old_commit = commit.parent(0)?;
        let old_tree = old_commit.tree()?;

        let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;

        let mut lines = vec![];
        diff.print(DiffFormat::NameOnly, |_, _, line| {
            lines.push(from_utf8(line.content()).map(|s| s.trim().to_owned()));
            true
        })?;

        let lines: Result<Vec<String>, Utf8Error> = lines.into_iter().collect();
        let lines = lines?;

        let filtered_lines: Vec<_> = lines
            .iter()
            .filter(|l| match &workspace.files_inclusion_mode {
                FilesIncludeAllFirst {
                    excludes,
                    includes_finally,
                } => {
                    let is_excluded = excludes.iter().any(|r| r.is_match(l));
                    if is_excluded {
                        includes_finally.iter().any(|r| r.is_match(l))
                    } else {
                        true
                    }
                }
                FilesExcludeAllFirst {
                    includes,
                    excludes_finally,
                } => {
                    let is_included = includes.iter().any(|r| r.is_match(l));
                    if is_included {
                        !excludes_finally.iter().any(|r| r.is_match(l))
                    } else {
                        false
                    }
                }
            })
            .collect();

        if !filtered_lines.is_empty() {
            println!("{}..{}", old_commit.id(), commit_id);
            println!(
                "Commit: {}\nAuthor: {}\nMessage: {}Files:",
                commit.id(),
                commit.author(),
                commit.message().unwrap_or("(No message)")
            );

            for l in filtered_lines {
                println!("{l}");
            }
            println!()
        }
    }

    Ok(())
}
