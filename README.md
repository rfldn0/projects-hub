# Projects Hub

A static index of everything I'm building, paused on, or have set down. One JSON file in, a few
kilobytes of plain HTML out. Built by a small Rust generator and deployed with GitHub Pages.

```sh
cargo run --release          # data/projects.json -> dist/
cargo test                   # unit tests + a full build of fixtures/sample.json
```

Open `dist/index.html` straight from disk; every link is relative and nothing is fetched at runtime.

## Layout

| Path | What it is |
| --- | --- |
| `data/projects.json` | The only file to edit when projects change |
| `src/model.rs` | JSON schema and validation |
| `src/view.rs` | Derived values: progress, current milestone, ordering |
| `src/render/` | One module per page (`home`, `detail`, `log`) plus the shared `layout` and `parts` |
| `styles/*.css` | Stylesheet partials, bundled in name order into `assets/site.css` |
| `static/` | Copied to `dist/assets/` as-is: `site.js`, favicon, images |
| `design/` | The original design file and reference art |
| `.github/workflows/static.yml` | Test, build and deploy on every push to `main` |

## Editing projects

```json
{
  "id": "ledger-os",
  "name": "Ledger OS",
  "desc": "One sentence.",
  "status": "Active",
  "note": "Why it's paused or abandoned (optional).",
  "started": "Sep 2026",
  "tags": ["rust", "tauri"],
  "repo": "https://github.com/...",
  "demo": "",
  "milestones": [{ "title": "Desktop shell", "done": true }, { "title": "Installer" }],
  "log": [{ "date": "2026-10-20", "text": "Migrated to Tauri 2" }]
}
```

- `status`: `Active`, `Paused` (or `Postponed`), `Abandoned`, `Done`.
- Projects are ordered by their newest log entry; the log page groups every entry by month.
- `site.progress` can be `"bar"` (default) or `"ring"`.
- Images are optional: `static/hero.*` (home), `static/log.*` (log page), and
  `static/covers/<id>.*` for a project cover. Without one, a hatched placeholder is drawn.

## Deploying

Push to `main` with **Settings → Pages → Source** set to **GitHub Actions**.
