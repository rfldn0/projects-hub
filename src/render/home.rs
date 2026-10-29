//! Home: hero, status filters and the scroll of project panels.

use crate::assets::Assets;
use crate::html::esc;
use crate::model::{Progress, Status};
use crate::view::View;

use super::parts;

pub fn render(views: &[View], progress: Progress, assets: &Assets) -> String {
    let panels: String = views.iter().enumerate().map(|(i, v)| panel(v, i == 0, progress, assets)).collect();
    let dots: String = views.iter().enumerate().map(|(i, v)| dot(v, i == 0)).collect();
    format!(
        r#"{hero}
<div class="toolbar">
<div class="chips" role="group" aria-label="Filter by status">{chips}</div>
<div class="hint"><span class="only-wide">drag, ← → keys or click a panel</span><span class="only-narrow">swipe →</span></div>
</div>
<div class="track" data-track>
<div class="track-cap"></div>
{panels}<div class="track-cap"></div>
</div>
<div class="dots">{dots}</div>"#,
        hero = hero(assets),
        chips = chips(views),
    )
}

fn hero(assets: &Assets) -> String {
    let art = match &assets.hero {
        Some(file) => format!(r#"<div class="night-art" style="background-image:url('assets/{}')"></div>"#, esc(file)),
        None => r#"<div class="night-art night-art--drawn"></div>"#.into(),
    };
    format!(
        r#"<section class="night">
{art}
<div class="night-copy">
<div><div class="eyebrow">Project index · 進行中</div><h1>Works in progress</h1></div>
<p>What I'm building, how far along each project is, and what I worked on most recently.</p>
</div>
</section>"#
    )
}

fn chips(views: &[View]) -> String {
    let chip = |slug: &str, label: &str, n: usize, on: bool| {
        format!(r#"<button type="button" class="chip" data-filter="{slug}" aria-pressed="{on}">{label} <span>{n}</span></button>"#)
    };
    let mut out = chip("all", "All", views.len(), true);
    for status in Status::ALL {
        let n = views.iter().filter(|v| v.p.status == status).count();
        if n > 0 {
            out += &chip(status.slug(), status.label(), n, false);
        }
    }
    out
}

fn panel(v: &View, selected: bool, progress: Progress, assets: &Assets) -> String {
    let p = v.p;
    let href = format!("projects/{}.html", esc(&p.id));
    let (meter, line) = match progress {
        Progress::Bar => (parts::big_pct(v.pct, "pct"), parts::bar(v.pct, "")),
        Progress::Ring => (parts::ring(v.pct), String::new()),
    };
    let latest = v
        .latest()
        .map(|e| format!(r#"<div class="latest"><span class="mono">LATEST · {}</span><span>{}</span></div>"#, e.date.short(), esc(&e.text)))
        .unwrap_or_default();
    let focus: String = v.focus().map(|i| parts::milestone_mark(v, i)).collect();
    format!(
        r#"<article class="panel{sel}" data-status="{status}" data-id="{id}" data-href="{href}" tabindex="0" aria-label="{name}">
<div class="panel-main">
<div class="panel-head"><h2 class="v-title">{name}</h2><div class="panel-meta">{badge}{meter}</div></div>
{line}
<p class="panel-desc">{desc}</p>
{latest}
<a class="panel-cta" href="{href}"><span class="only-wide">Click to unroll →</span><span class="only-narrow">Tap to unroll →</span></a>
</div>
<div class="panel-more">
{cover}
<div class="mono">{done} / {total} MILESTONES · {tags}</div>
{note}<ul class="marks">{focus}</ul>
<div class="actions"><a class="btn btn--seal" href="{href}">Open project →</a>{repo}</div>
</div>
</article>
"#,
        sel = if selected { " is-sel" } else { "" },
        status = p.status.slug(),
        id = esc(&p.id),
        name = esc(&p.name),
        badge = parts::status(p.status),
        desc = esc(&p.desc),
        cover = parts::cover(v, assets, "", "cover--panel"),
        done = v.done,
        total = v.total(),
        tags = parts::tags(v),
        note = parts::note(v),
        repo = parts::link(&p.repo, "Repo ↗", "btn--line"),
    )
}

fn dot(v: &View, selected: bool) -> String {
    format!(
        r#"<button type="button" class="dot" data-id="{id}" data-status="{status}" aria-label="{name}" aria-current="{selected}"></button>"#,
        id = esc(&v.p.id),
        status = v.p.status.slug(),
        name = esc(&v.p.name),
    )
}
