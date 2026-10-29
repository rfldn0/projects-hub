//! Optional images in `static/`: `hero.*`, `log.*` and `covers/<project-id>.*`.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const IMAGE_EXTS: [&str; 5] = ["avif", "webp", "jpg", "jpeg", "png"];

#[derive(Default)]
pub struct Assets {
    pub hero: Option<String>,
    pub log: Option<String>,
    covers: HashMap<String, String>,
}

impl Assets {
    /// Missing directories just mean no images; the pages fall back to drawn placeholders.
    pub fn scan(dir: &Path) -> Self {
        let top = images(dir);
        Assets {
            hero: top.get("hero").cloned(),
            log: top.get("log").cloned(),
            covers: images(&dir.join("covers"))
                .into_iter()
                .map(|(stem, file)| (stem, format!("covers/{file}")))
                .collect(),
        }
    }

    /// Path relative to the assets folder.
    pub fn cover(&self, id: &str) -> Option<&str> {
        self.covers.get(id).map(String::as_str)
    }
}

/// stem -> file name for every image directly inside `dir`.
fn images(dir: &Path) -> HashMap<String, String> {
    let Ok(entries) = fs::read_dir(dir) else { return HashMap::new() };
    let mut found = HashMap::new();
    for path in entries.flatten().map(|e| e.path()) {
        let (Some(stem), Some(ext), Some(name)) = (
            path.file_stem().and_then(|s| s.to_str()),
            path.extension().and_then(|s| s.to_str()),
            path.file_name().and_then(|s| s.to_str()),
        ) else {
            continue;
        };
        if IMAGE_EXTS.contains(&ext.to_ascii_lowercase().as_str()) {
            found.entry(stem.to_owned()).or_insert_with(|| name.to_owned());
        }
    }
    found
}

/// Concatenates `*.css` partials in file-name order (hence the numeric prefixes),
/// so the site ships one stylesheet while the source stays in small files.
pub fn bundle_css(dir: &Path) -> io::Result<String> {
    let mut files: Vec<_> = fs::read_dir(dir)?
        .map(|e| e.map(|e| e.path()))
        .collect::<io::Result<_>>()?;
    files.retain(|p| p.extension().is_some_and(|e| e == "css"));
    files.sort();
    let mut css = String::new();
    for file in files {
        css += &fs::read_to_string(file)?;
        css.push('\n');
    }
    Ok(css)
}

pub fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    if !from.is_dir() {
        return Ok(());
    }
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
