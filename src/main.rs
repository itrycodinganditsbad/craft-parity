use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};

use craft_parity::{fix, git, html, report, scan};

/// Shows which Craft apps carry a fix that already landed in a sibling app.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Folder holding one checkout per app, named as in the app list.
    #[arg(long, default_value = "..")]
    root: PathBuf,
    /// Folder of fix fingerprints.
    #[arg(long, default_value = "fixes")]
    fixes: PathBuf,
    /// The app list.
    #[arg(long, default_value = "apps.toml")]
    apps: PathBuf,
    /// Ref to read in every checkout. Default: the upstream default branch
    /// (origin/HEAD, then origin/main, origin/master, HEAD).
    #[arg(long)]
    rev: Option<String>,
    /// `git fetch origin` in each checkout first.
    #[arg(long)]
    fetch: bool,
    /// Shallow-clone apps that have no checkout under --root.
    #[arg(long)]
    clone: bool,
    /// Only these fixes (by id); repeatable.
    #[arg(long = "only")]
    only: Vec<String>,
    #[arg(long, value_enum, default_value_t = Format::Markdown)]
    format: Format,
    /// Write the report here instead of standard output.
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Markdown,
    Json,
    /// One self-contained page.
    Html,
}

fn run(args: Args) -> Result<bool, String> {
    let apps = fix::load_apps(&args.apps)?;
    let mut fixes = fix::load_fixes(&args.fixes, &apps)?;
    if !args.only.is_empty() {
        if let Some(id) = args.only.iter().find(|id| !fixes.iter().any(|f| &f.id == *id)) {
            return Err(format!("no fix `{id}` in {}", args.fixes.display()));
        }
        fixes.retain(|f| args.only.contains(&f.id));
    }

    for app in &apps {
        let repo = args.root.join(&app.name);
        if !repo.join(".git").exists() {
            if args.clone {
                eprintln!("cloning {}", app.repo);
                git::clone(&format!("https://github.com/{}.git", app.repo), &repo)?;
            }
        } else if args.fetch {
            eprintln!("fetching {}", app.name);
            if let Err(e) = git::fetch(&repo) {
                eprintln!("warning: {}: {e}", app.name);
            }
        }
    }

    let revs: Vec<&str> = match &args.rev {
        Some(rev) => vec![rev.as_str()],
        None => scan::DEFAULT_REVS.to_vec(),
    };
    let located: Vec<_> = apps.iter().map(|a| scan::locate(&args.root, a, &revs)).collect();
    for a in &located {
        if let Err(e) = &a.commit {
            eprintln!("warning: {}: {e}", a.app.name);
        }
    }
    let report = scan::scan(&args.root, &located, &fixes);

    let text = match args.format {
        Format::Markdown => report::markdown(&report),
        Format::Html => html::html(&report, concat!(env!("CARGO_PKG_REPOSITORY"), "/blob/main")),
        Format::Json => serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n",
    };
    match &args.out {
        Some(path) => fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?,
        None => print!("{text}"),
    }

    for row in &report.rows {
        for app in &row.stale {
            eprintln!("error: fingerprint `{}` reads {app} as not fixed, but {app} is listed in known_fixed", row.fix.id);
        }
    }
    Ok(!report.is_stale())
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(2),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
