//! One page per project: progress, milestones, activity, prev/next.

use crate::assets::Assets;
use crate::html::esc;
use crate::view::View;

use super::parts;

pub fn render(views: &[View], i: usize, assets: &Assets) -> String {
    let v = &views[i];
    let p = v.p;
    let n = views.len();
    let prev = &views[(i + n - 1) % n];
    let next = &views[(i + 1) % n];
    let milestones: String = (0..v.total()).map(|j| parts::milestone(v, j)).collect();
    let activity: String = v
        .log
        .iter()
        .map(|e| {
            format!(
                r#"<li class="entry"><time class="mono" datetime="{iso}">{short}</time><span>{text}</span></li>"#,
                iso = e.date,
                short = e.date.short(),
                text = esc(&e.text)
            )
        })
        .collect();

    format!(
        r#"<div class="sheet ink-in">
<div class="crumbs"><a href="../index.html">← All projects</a><span class="mono">{pos} / {n}</span></div>
<div class="lead">
<h1 class="v-title v-title--lead">{name}</h1>
<div class="lead-body">
<span class="status-line">{badge}<span class="status status--plain"> · started {started}</span></span>
<p class="lead-desc">{desc}</p>
{note}<div class="lead-pct">{pct}<span class="mono">{done} OF {total} MILESTONES</span></div>
{bar}
<div class="actions">{repo}{demo}<span class="mono">{tags}</span></div>
</div>
</div>
{cover}
<div class="columns">
<section><h3 class="col-head">Milestones <span class="mono">{done} / {total}</span></h3><ul class="plain">{milestones}</ul></section>
<section><h3 class="col-head">Activity</h3><ol class="plain">{activity}</ol></section>
</div>
<nav class="pager" aria-label="Projects">
<a href="{prev_id}.html" data-prev><span class="mono">PREVIOUS</span>← {prev_name}</a>
<a href="{next_id}.html" data-next class="pager-next"><span class="mono">NEXT</span>{next_name} →</a>
</nav>
</div>"#,
        pos = i + 1,
        name = esc(&p.name),
        badge = parts::status(p.status),
        started = esc(&p.started),
        desc = esc(&p.desc),
        note = parts::note(v),
        pct = parts::big_pct(v.pct, "pct pct--xl"),
        done = v.done,
        total = v.total(),
        bar = parts::bar(v.pct, "bar--lead"),
        repo = parts::link(&p.repo, "Repository ↗", "btn--ink"),
        demo = parts::link(&p.demo, "Live demo ↗", "btn--line"),
        tags = parts::tags(v),
        cover = parts::cover(v, assets, "../", "cover--lead"),
        prev_id = esc(&prev.p.id),
        prev_name = esc(&prev.p.name),
        next_id = esc(&next.p.id),
        next_name = esc(&next.p.name),
    )
}
