//! The shape of projects.json, plus the checks serde can't express.

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::date::Date;

#[derive(Deserialize)]
pub struct Data {
    pub site: Site,
    pub projects: Vec<Project>,
}

#[derive(Deserialize)]
pub struct Site {
    pub owner: String,
    #[serde(default)]
    pub github: String,
    /// Falls back to the newest log entry when absent.
    #[serde(default)]
    pub updated: Option<Date>,
    #[serde(default)]
    pub progress: Progress,
}

/// How a panel shows completion: the design's bar, or its ring variant.
#[derive(Deserialize, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Progress {
    #[default]
    Bar,
    Ring,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    #[serde(alias = "Postponed")]
    Paused,
    Abandoned,
    Done,
}

impl Status {
    pub const ALL: [Status; 4] = [Status::Active, Status::Paused, Status::Abandoned, Status::Done];

    pub fn label(self) -> &'static str {
        match self {
            Status::Active => "Active",
            Status::Paused => "Paused",
            Status::Abandoned => "Abandoned",
            Status::Done => "Done",
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Status::Active => "active",
            Status::Paused => "paused",
            Status::Abandoned => "abandoned",
            Status::Done => "done",
        }
    }
}

#[derive(Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub status: Status,
    /// Why a project is paused or abandoned.
    #[serde(default)]
    pub note: Option<String>,
    pub started: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub repo: String,
    #[serde(default)]
    pub demo: String,
    #[serde(default)]
    pub milestones: Vec<Milestone>,
    #[serde(default)]
    pub log: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct Milestone {
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

#[derive(Deserialize)]
pub struct Entry {
    pub date: Date,
    pub text: String,
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Date::try_from(String::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}

pub fn load(path: &Path) -> Result<Data, Box<dyn Error>> {
    let text = fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let data: Data = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    validate(&data)?;
    Ok(data)
}

fn validate(data: &Data) -> Result<(), String> {
    if data.projects.is_empty() {
        return Err("projects.json has no projects".into());
    }
    let mut seen = HashSet::new();
    for p in &data.projects {
        let slug_ok = !p.id.is_empty()
            && p.id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
        if !slug_ok {
            return Err(format!("project id `{}` must be lowercase letters, digits and dashes", p.id));
        }
        if !seen.insert(p.id.as_str()) {
            return Err(format!("duplicate project id `{}`", p.id));
        }
        if p.name.trim().is_empty() {
            return Err(format!("project `{}` has an empty name", p.id));
        }
    }
    Ok(())
}
