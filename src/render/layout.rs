//! The document shell every page shares: head, masthead, footer.

use crate::html::{esc, external_url};
use crate::model::Site;

const FONTS: &str = "https://fonts.googleapis.com/css2?family=Noto+Serif+SC:wght@400;600;900&family=Noto+Sans+SC:wght@400;500&family=IBM+Plex+Mono:wght@400;500&display=swap";

#[derive(PartialEq, Eq)]
pub enum Nav {
    Projects,
    Log,
}

pub struct Page<'a> {
    pub title: String,
    pub description: String,
    /// Relative path back to the site root ("" or "../"), so pages open from disk.
    pub root: &'a str,
    pub nav: Nav,
    pub body: String,
}

pub struct Shell<'a> {
    pub site: &'a Site,
    pub updated: String,
}

impl Shell<'_> {
    pub fn page(&self, page: Page) -> String {
        let root = page.root;
        let current = |nav: Nav| if page.nav == nav { r#" aria-current="page""# } else { "" };
        let github = external_url(&self.site.github)
            .map(|url| format!(r#"<a href="{url}" rel="noopener">GitHub ↗</a>"#))
            .unwrap_or_default();
        format!(
            r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<meta name="description" content="{description}">
<meta name="color-scheme" content="light dark">
<meta name="theme-color" content="#f3eee3" media="(prefers-color-scheme: light)">
<meta name="theme-color" content="#12100f" media="(prefers-color-scheme: dark)">
<link rel="icon" href="{root}assets/favicon.svg" type="image/svg+xml">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link rel="stylesheet" href="{FONTS}">
<link rel="stylesheet" href="{root}assets/site.css">
<script>document.documentElement.classList.add("js")</script>
<script src="{root}assets/site.js" defer></script>
</head>
<body>
<a class="skip" href="#main">Skip to content</a>
<header class="top">
<a class="brand" href="{root}index.html"><span class="seal" aria-hidden="true">印</span><span>{owner}</span></a>
<nav class="nav" aria-label="Site"><a href="{root}index.html"{cur_projects}>Projects</a><a href="{root}log.html"{cur_log}>Log</a>{github}</nav>
</header>
<main id="main">
{body}
</main>
<footer class="foot"><span>Updated {updated} · data/projects.json</span><span>Built with Rust · Deployed with GitHub Pages</span></footer>
</body>
</html>
"##,
            title = esc(&page.title),
            description = esc(&page.description),
            owner = esc(&self.site.owner),
            cur_projects = current(Nav::Projects),
            cur_log = current(Nav::Log),
            body = page.body,
            updated = esc(&self.updated),
        )
    }
}
