//! projects-hub: renders `data/projects.json` into a static, file://-safe site.

mod assets;
mod build;
mod date;
mod html;
mod model;
mod render;
mod view;

use std::process::ExitCode;

const USAGE: &str = "usage: projects-hub [--data FILE] [--static DIR] [--styles DIR] [--out DIR]";

fn main() -> ExitCode {
    let config = match build::Config::from_args(std::env::args().skip(1)) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("{msg}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    match build::run(&config) {
        Ok(pages) => {
            println!("built {pages} pages into {}", config.out.display());
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
