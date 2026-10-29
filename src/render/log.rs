//! Log: every entry from every project, newest first, grouped by month.

use std::cmp::Reverse;

use crate::assets::Assets;
use crate::html::esc;
use crate::model::Entry;
use crate::view::View;

pub fn render(views: &[View], assets: &Assets) -> String {
    let mut entries: Vec<(&Entry, &View)> = views.iter().flat_map(|v| v.log.iter().map(move |e| (*e, v))).collect();
    entries.sort_by_key(|(e, _)| Reverse(e.date));

    let mut body = String::new();
    let mut month = String::new();
    for (e, v) in entries {
        let this_month = e.date.month_year();
        if this_month != month {
            if !month.is_empty() {
                body += "</ol>\n";
            }
            body += &format!(r#"<h2 class="month">{this_month}</h2><ol class="plain log">"#);
            month = this_month;
        }
        body += &format!(
            r#"<li><a class="log-row" href="projects/{id}.html"><time class="mono" datetime="{iso}">{short}</time><span>{text}</span><span class="log-project status--{status}">{name}</span></a></li>"#,
            id = esc(&v.p.id),
            iso = e.date,
            short = e.date.short(),
            text = esc(&e.text),
            status = v.p.status.slug(),
            name = esc(&v.p.name),
        );
    }
    if !month.is_empty() {
        body += "</ol>\n";
    }

    let art = match &assets.log {
        Some(file) => format!(r#"<div class="night-art night-art--log" style="background-image:url('assets/{}')"></div>"#, esc(file)),
        None => String::new(),
    };
    format!(
        r#"<section class="night night--log">
{art}
<div class="night-copy"><div><div class="eyebrow">All activity · 日誌</div><h1>Log</h1></div></div>
</section>
<div class="sheet sheet--narrow ink-in">{body}</div>"#
    )
}
