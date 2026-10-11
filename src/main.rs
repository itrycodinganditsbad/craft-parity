use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};

use craft_parity::index::Index;
use craft_parity::{fix, git, github, html, report, scan, sync};

/// Shows which Craft apps carry a fix that already landed in a sibling app.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check every fingerprint against every app's checkout and write the report.
    Scan(ScanArgs),
    /// Bring the issue and pull request index up to date with GitHub.
    Sync(SyncArgs),
}

#[derive(ClapArgs)]
struct SyncArgs {
    /// Folder holding the index (the checkout of the `data` branch).
    #[arg(long, default_value = "data")]
    data: PathBuf,
    /// The app list.
    #[arg(long, default_value = "apps.toml")]
    apps: PathBuf,
    /// Re-read everything instead of what changed since the last sync.
    #[arg(long)]
    full: bool,
    /// Only these apps (by name); repeatable.
    #[arg(long = "app")]
    only: Vec<String>,
}

#[derive(ClapArgs)]
struct ScanArgs {
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

fn run_sync(args: SyncArgs) -> Result<bool, String> {
    let mut apps = fix::load_apps(&args.apps)?;
    if !args.only.is_empty() {
        if let Some(name) = args.only.iter().find(|n| !apps.iter().any(|a| &a.name == *n)) {
            return Err(format!("no app `{name}` in {}", args.apps.display()));
        }
        apps.retain(|a| args.only.contains(&a.name));
    }
    let gh = github::Client::new(github::token()?);
    let counts = sync::sync_all(&gh, &Index::new(&args.data), &apps, args.full)?;
    for (app, c) in counts {
        eprintln!(
            "{app}: {} PRs read, {} changed; {} issues read, {} changed{}",
            c.prs_seen,
            c.prs_changed,
            c.issues_seen,
            c.issues_changed,
            if c.removed > 0 { format!("; {} removed", c.removed) } else { String::new() }
        );
    }
    Ok(true)
}

fn run_scan(args: ScanArgs) -> Result<bool, String> {
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
    let result = match Cli::parse().command {
        Command::Scan(args) => run_scan(args),
        Command::Sync(args) => run_sync(args),
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(2),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
