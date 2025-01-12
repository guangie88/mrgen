use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct ProcessingConf {
    #[serde(with = "serde_regex")]
    search: Regex,

    replace: String,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConf {
    #[serde(default, with = "serde_regex")]
    pre_captures: Option<Vec<Regex>>,

    #[serde(default, with = "serde_regex")]
    pre_captures_after_trim: Option<Regex>,

    #[serde(default, with = "serde_regex")]
    type_captures: Option<Vec<Regex>>,

    #[serde(default, with = "serde_regex")]
    type_captures_after_trim: Option<Regex>,

    type_captures_allow_breaking_change_group: Option<bool>,

    #[serde(default, with = "serde_regex")]
    breaking_change_line_captures: Option<Vec<Regex>>,

    #[serde(default, with = "serde_regex")]
    breaking_change_line_captures_after_trim: Option<Regex>,

    #[serde(default, with = "serde_regex")]
    title_left_trim: Option<Regex>,

    #[serde(default, with = "serde_regex")]
    title_right_trim: Option<Regex>,

    supported_types: Option<HashMap<String, String>>,

    headings: Option<HashMap<String, String>>,

    others_heading: Option<String>,

    breaking_changes_heading: Option<String>,

    capitalize_title_first_char: Option<bool>,

    preprocessing: Option<ProcessingConf>,

    postprocessing: Option<ProcessingConf>,

    tag_prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilesInclusionMode {
    FilesIncludeAllFirst {
        #[serde(with = "serde_regex")]
        excludes: Vec<Regex>,

        #[serde(with = "serde_regex")]
        includes_finally: Vec<Regex>,
    },

    FilesExcludeAllFirst {
        #[serde(with = "serde_regex")]
        includes: Vec<Regex>,

        #[serde(with = "serde_regex")]
        excludes_finally: Vec<Regex>,
    },
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceConf {
    path: PathBuf,

    #[serde(flatten)]
    files_inclusion_mode: FilesInclusionMode,

    #[serde(flatten)]
    general: GeneralConf,
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    #[serde(flatten)]
    global: GeneralConf,

    workspaces: Vec<WorkspaceConf>,
}
