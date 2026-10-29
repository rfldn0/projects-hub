//! Orchestrates a build: load data, render pages, write them with the static assets.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{assets, model, render, view};

/// Written into the output directory so a rebuild only ever wipes a folder we created.
const MARKER: &str = ".projects-hub";

pub struct Config {
    pub data: PathBuf,
    pub static_dir: PathBuf,
    pub styles: PathBuf,
    pub out: PathBuf,
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut config = Config {
            data: "data/projects.json".into(),
            static_dir: "static".into(),
            styles: "styles".into(),
            out: "dist".into(),
        };
        while let Some(flag) = args.next() {
            let slot = match flag.as_str() {
                "--data" => &mut config.data,
                "--static" => &mut config.static_dir,
                "--styles" => &mut config.styles,
                "--out" => &mut config.out,
                _ => return Err(format!("unknown argument `{flag}`")),
            };
            *slot = args.next().ok_or(format!("`{flag}` needs a value"))?.into();
        }
        Ok(config)
    }
}

pub fn run(config: &Config) -> Result<usize, Box<dyn Error>> {
    let data = model::load(&config.data)?;
    let assets = assets::Assets::scan(&config.static_dir);
    let views = view::build(&data.projects);
    let pages = render::site(&data, &views, &assets);
    let css = assets::bundle_css(&config.styles)
        .map_err(|e| format!("reading styles in {}: {e}", config.styles.display()))?;

    prepare_out(&config.out)?;
    for (path, html) in &pages {
        let target = config.out.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, html)?;
    }
    assets::copy_dir(&config.static_dir, &config.out.join("assets"))?;
    fs::write(config.out.join("assets/site.css"), css)?;
    Ok(pages.len())
}

fn prepare_out(out: &Path) -> Result<(), Box<dyn Error>> {
    if out.exists() {
        let ours = out.join(MARKER).exists();
        let empty = fs::read_dir(out)?.next().is_none();
        if !ours && !empty {
            return Err(format!("refusing to overwrite `{}`: not a previous build", out.display()).into());
        }
        fs::remove_dir_all(out)?;
    }
    fs::create_dir_all(out)?;
    fs::write(out.join(MARKER), "")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_fixture_site() {
        let out = std::env::temp_dir().join("projects-hub-test-dist");
        let config = Config {
            data: "fixtures/sample.json".into(),
            static_dir: "static".into(),
            styles: "styles".into(),
            out: out.clone(),
        };
        assert_eq!(run(&config).unwrap(), 2 + 4);

        let read = |p: &str| fs::read_to_string(out.join(p)).unwrap();
        let log = read("log.html");
        assert!(log.contains("Paused for exams &lt;b&gt;"), "log text is escaped");
        assert!(log.find("December 2025") > log.find("October 2026"), "months newest first");
        assert!(!read("projects/old-bot.html").contains("javascript:"), "unsafe links dropped");
        assert!(read("index.html").contains(r#"data-status="paused""#), "Postponed aliases Paused");
        assert!(read("assets/site.css").contains("--seal"), "styles bundled");

        // A second build may replace its own output, but never a foreign folder.
        assert!(run(&config).is_ok());
        let foreign = std::env::temp_dir().join("projects-hub-test-foreign");
        fs::create_dir_all(&foreign).unwrap();
        fs::write(foreign.join("keep.txt"), "x").unwrap();
        assert!(run(&Config { out: foreign.clone(), ..config }).is_err());
        assert!(foreign.join("keep.txt").exists());
    }
}
