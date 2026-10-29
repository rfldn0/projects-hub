//! Small fragments reused across pages.

use crate::assets::Assets;
use crate::html::{esc, external_url};
use crate::model::Status;
use crate::view::{Step, View};

pub fn status(status: Status) -> String {
    format!(r#"<span class="status status--{}">● {}</span>"#, status.slug(), status.label())
}

pub fn bar(pct: u32, class: &str) -> String {
    format!(r#"<div class="bar {class}"><div class="bar-fill" style="width:{pct}%"></div></div>"#)
}

pub fn ring(pct: u32) -> String {
    format!(r#"<div class="ring" style="--pct:{pct}%"><span>{pct}</span></div>"#)
}

pub fn big_pct(pct: u32, class: &str) -> String {
    format!(r#"<span class="{class}">{pct}<small>%</small></span>"#)
}

pub fn tags(v: &View) -> String {
    esc(&v.p.tags.join(" · "))
}

/// A real image when `static/covers/<id>.*` exists, else a drawn red-moon header
/// whose moon sits in a spot derived from the id, so each project looks distinct.
pub fn cover(v: &View, assets: &Assets, root: &str, class: &str) -> String {
    match assets.cover(&v.p.id) {
        Some(file) => format!(
            r#"<img class="cover {class}" src="{root}assets/{file}" alt="" loading="lazy" decoding="async">"#,
            file = esc(file)
        ),
        None => {
            let h = fnv1a(&v.p.id);
            format!(
                r#"<div class="cover cover--drawn {class}" style="--mx:{x}%;--my:{y}%" aria-hidden="true"><span>{name}</span></div>"#,
                x = 30 + h % 55,
                y = 32 + (h >> 8) % 22,
                name = esc(&v.p.name)
            )
        }
    }
}

fn fnv1a(s: &str) -> u32 {
    s.bytes().fold(0x811c_9dc5, |h, b| (h ^ u32::from(b)).wrapping_mul(0x0100_0193))
}

pub fn milestone(v: &View, i: usize) -> String {
    let step = v.step(i);
    let tag = match step {
        Step::Done => "DONE",
        Step::Now => "NOW",
        Step::Todo => "",
    };
    format!(
        r#"<li class="ms ms--{slug}"><span class="ms-box"></span>{title}<span class="ms-tag">{tag}</span></li>"#,
        slug = step.slug(),
        title = esc(&v.p.milestones[i].title)
    )
}

/// Compact milestone line for the unrolled panel.
pub fn milestone_mark(v: &View, i: usize) -> String {
    let step = v.step(i);
    let mark = if step == Step::Done { "■" } else { "□" };
    format!(r#"<li class="mark mark--{}">{mark}&ensp;{}</li>"#, step.slug(), esc(&v.p.milestones[i].title))
}

/// Repo / demo buttons; links that aren't http(s) are dropped.
pub fn link(href: &str, label: &str, class: &str) -> String {
    external_url(href)
        .map(|url| format!(r#"<a class="btn {class}" href="{url}" rel="noopener">{label}</a>"#))
        .unwrap_or_default()
}

pub fn note(v: &View) -> String {
    v.p.note
        .as_deref()
        .filter(|n| !n.trim().is_empty())
        .map(|n| format!(r#"<p class="note">{}</p>"#, esc(n)))
        .unwrap_or_default()
}
