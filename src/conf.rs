use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct ProcessingConf {
    #[serde(with = "serde_regex")]
    pub search: Regex,

    pub replace: String,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConf {
    #[serde(default, with = "serde_regex")]
    pub pre_captures: Option<Vec<Regex>>,

    #[serde(default, with = "serde_regex")]
    pub pre_captures_after_trim: Option<Regex>,

    #[serde(default, with = "serde_regex")]
    pub type_captures: Option<Vec<Regex>>,

    #[serde(default, with = "serde_regex")]
    pub type_captures_after_trim: Option<Regex>,

    pub type_captures_allow_breaking_change_group: Option<bool>,

    #[serde(default, with = "serde_regex")]
    pub breaking_change_line_captures: Option<Vec<Regex>>,

    #[serde(default, with = "serde_regex")]
    pub breaking_change_line_captures_after_trim: Option<Regex>,

    #[serde(default, with = "serde_regex")]
    pub title_left_trim: Option<Regex>,

    #[serde(default, with = "serde_regex")]
    pub title_right_trim: Option<Regex>,

    pub supported_types: Option<HashMap<String, String>>,

    pub headings: Option<HashMap<String, String>>,

    pub others_heading: Option<String>,

    pub breaking_changes_heading: Option<String>,

    pub capitalize_title_first_char: Option<bool>,

    pub preprocessing: Option<ProcessingConf>,

    pub postprocessing: Option<ProcessingConf>,

    pub tag_prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilesInclusionMode {
    FilesIncludeAllFirst {
        #[serde(default, with = "serde_regex")]
        excludes: Vec<Regex>,

        #[serde(default, with = "serde_regex")]
        includes_finally: Vec<Regex>,
    },

    FilesExcludeAllFirst {
        #[serde(default, with = "serde_regex")]
        includes: Vec<Regex>,

        #[serde(default, with = "serde_regex")]
        excludes_finally: Vec<Regex>,
    },
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceConf {
    pub path: PathBuf,

    #[serde(flatten)]
    pub files_inclusion_mode: FilesInclusionMode,

    #[serde(flatten)]
    pub general: GeneralConf,
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    #[serde(flatten)]
    pub global: GeneralConf,

    pub workspaces: Vec<WorkspaceConf>,
}
