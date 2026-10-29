//! Turns derived views into finished HTML pages, keyed by output path.

mod detail;
mod home;
mod layout;
mod log;
mod parts;

use std::path::PathBuf;

use crate::assets::Assets;
use crate::model::Data;
use crate::view::View;
use layout::{Nav, Page, Shell};

pub fn site(data: &Data, views: &[View], assets: &Assets) -> Vec<(PathBuf, String)> {
    let updated = data
        .site
        .updated
        .or_else(|| views.iter().filter_map(|v| v.latest()).map(|e| e.date).max())
        .map(|d| d.long())
        .unwrap_or_default();
    let shell = Shell { site: &data.site, updated };
    let owner = &data.site.owner;

    let mut pages = vec![
        (
            PathBuf::from("index.html"),
            shell.page(Page {
                title: format!("{owner} — Projects"),
                description: "What I'm building, how far along each project is, and what I worked on most recently.".into(),
                root: "",
                nav: Nav::Projects,
                body: home::render(views, data.site.progress, assets),
            }),
        ),
        (
            PathBuf::from("log.html"),
            shell.page(Page {
                title: format!("Log — {owner}"),
                description: "Every update across every project, newest first.".into(),
                root: "",
                nav: Nav::Log,
                body: log::render(views, assets),
            }),
        ),
    ];
    for (i, v) in views.iter().enumerate() {
        pages.push((
            PathBuf::from("projects").join(format!("{}.html", v.p.id)),
            shell.page(Page {
                title: format!("{} — {owner}", v.p.name),
                description: v.p.desc.clone(),
                root: "../",
                nav: Nav::Projects,
                body: detail::render(views, i, assets),
            }),
        ));
    }
    pages
}
