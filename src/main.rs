use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};

use craft_parity::index::Index;
use craft_parity::{classify, cluster, draft, fix, git, github, html, report, scan, sync};

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
    /// Label fixes, open PRs and open issues with Claude, resuming where the last run stopped.
    Classify(ClassifyArgs),
    /// Group classified records that describe the same bug in different apps.
    Cluster(ClusterArgs),
    /// Draft a fingerprint for groups of the same bug and prove it on their own fix PRs.
    Draft(DraftArgs),
}

#[derive(ClapArgs)]
struct DraftArgs {
    #[arg(long, default_value = "data")]
    data: PathBuf,
    #[arg(long, default_value = "apps.toml")]
    apps: PathBuf,
    /// Folder holding one checkout per app; fix commits are fetched into them when missing.
    #[arg(long, default_value = "..")]
    root: PathBuf,
    /// The group containing this record id (e.g. designcraft/pr/66); repeatable.
    #[arg(long)]
    group: Vec<String>,
    /// Or the first N multi-app groups from candidates.json that have no draft yet.
    #[arg(long, default_value_t = 0)]
    top: usize,
    #[arg(long, default_value = "claude-opus-5-5")]
    model: String,
    #[arg(long, default_value = "high")]
    effort: String,
    #[arg(long, default_value_t = 3)]
    attempts: u32,
    /// Draft again even when a draft exists.
    #[arg(long)]
    redo: bool,
    /// Only check the apps that read missing in existing drafts (no new drafts).
    #[arg(long)]
    verify_only: bool,
}

#[derive(ClapArgs)]
struct ClusterArgs {
    #[arg(long, default_value = "data")]
    data: PathBuf,
    #[arg(long, default_value = "apps.toml")]
    apps: PathBuf,
    /// Similarity (0 to 1) at which two records join.
    #[arg(long, default_value_t = cluster::DEFAULT_THRESHOLD)]
    threshold: f64,
    /// Print this many candidates.
    #[arg(long, default_value_t = 25)]
    show: usize,
}

#[derive(ClapArgs)]
struct ClassifyArgs {
    /// Folder holding the index (the checkout of the `data` branch).
    #[arg(long, default_value = "data")]
    data: PathBuf,
    /// The app list.
    #[arg(long, default_value = "apps.toml")]
    apps: PathBuf,
    /// How to reach Claude. `claude-cli` runs `claude -p` on your logged-in subscription.
    #[arg(long, value_enum, default_value_t = Via::ClaudeCli)]
    via: Via,
    #[arg(long, default_value = "claude-opus-5-5")]
    model: String,
    #[arg(long, default_value = "low")]
    effort: String,
    /// Records per call.
    #[arg(long, default_value_t = 20)]
    batch_size: usize,
    /// Calls in flight at once.
    #[arg(long, default_value_t = 3)]
    jobs: usize,
    /// Stop after this many calls.
    #[arg(long)]
    max_calls: Option<usize>,
    /// Only these apps (by name); repeatable.
    #[arg(long = "app")]
    only: Vec<String>,
    /// Count what would be sent, then stop.
    #[arg(long)]
    dry_run: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Via {
    ClaudeCli,
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
    /// The data branch checkout; when given, the HTML report also lists the groups from
    /// `llm/candidates.json`.
    #[arg(long)]
    data: Option<PathBuf>,
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

fn run_classify(args: ClassifyArgs) -> Result<bool, String> {
    let mut apps = fix::load_apps(&args.apps)?;
    if !args.only.is_empty() {
        if let Some(name) = args.only.iter().find(|n| !apps.iter().any(|a| &a.name == *n)) {
            return Err(format!("no app `{name}` in {}", args.apps.display()));
        }
        apps.retain(|a| args.only.contains(&a.name));
    }
    let index = Index::new(&args.data);
    let store = classify::Store::new(&args.data);
    if args.dry_run {
        let mut total = 0;
        for app in &apps {
            let have = store.load(&app.name)?;
            let todo = classify::items(&index, app, &args.model)?.into_iter().filter(|i| have.get(&i.id).is_none_or(|c| c.input != i.hash)).count();
            eprintln!("{}: {todo} to classify", app.name);
            total += todo;
        }
        eprintln!("{total} records, {} calls of {}", total.div_ceil(args.batch_size.max(1)), args.batch_size);
        return Ok(true);
    }
    let Via::ClaudeCli = args.via;
    let workdir = std::env::temp_dir().join("craft-parity-claude");
    fs::create_dir_all(&workdir).map_err(|e| format!("{}: {e}", workdir.display()))?;
    let classifier = classify::ClaudeCli { model: args.model.clone(), effort: args.effort.clone(), workdir };
    let opts = classify::Options { batch_size: args.batch_size, jobs: args.jobs, max_calls: args.max_calls };
    let outcome = classify::run(&classifier, &index, &store, &apps, &opts)?;
    eprintln!(
        "{} of {} pending records classified in {} calls; {} tokens in, {} out (list-price estimate ${:.2})",
        outcome.classified, outcome.pending, outcome.calls, outcome.usage.input_tokens, outcome.usage.output_tokens, outcome.usage.cost_usd
    );
    if let Some(e) = outcome.stopped {
        return Err(format!("stopped early, run again to resume: {e}"));
    }
    Ok(true)
}

fn run_cluster(args: ClusterArgs) -> Result<bool, String> {
    let apps = fix::load_apps(&args.apps)?;
    let records = cluster::load(&Index::new(&args.data), &classify::Store::new(&args.data), &apps)?;
    let all = cluster::clusters(&records, args.threshold);
    let candidates: Vec<&cluster::Cluster> = all.iter().filter(|c| c.is_candidate()).collect();
    let path = args.data.join("llm").join("candidates.json");
    let text = serde_json::to_string_pretty(&candidates).map_err(|e| e.to_string())? + "\n";
    craft_parity::index::write_if_changed(&path, text.as_bytes())?;
    eprintln!("{} records, {} clusters, {} candidates -> {}", records.len(), all.len(), candidates.len(), path.display());
    for c in candidates.iter().take(args.show) {
        println!("\n[{}] fixed in: {} | open in: {}", c.area, c.fixed_in.join(", "), c.open_in.join(", "));
        for m in &c.members {
            println!("  {:?} {:24} {}", m.state, m.id, m.symptom);
        }
    }
    Ok(true)
}

fn run_draft(args: DraftArgs) -> Result<bool, String> {
    let apps = fix::load_apps(&args.apps)?;
    let path = args.data.join("llm").join("candidates.json");
    let groups: Vec<cluster::Cluster> = serde_json::from_str(&fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let dir = args.data.join("llm").join("drafts");
    let exists = |g: &cluster::Cluster| draft::key(g).is_some_and(|k| dir.join(format!("{k}.json")).exists());
    let mut chosen: Vec<&cluster::Cluster> = Vec::new();
    for id in &args.group {
        let g = groups.iter().find(|g| g.members.iter().any(|m| &m.id == id)).ok_or_else(|| format!("no candidate group holds {id}"))?;
        chosen.push(g);
    }
    let multi = |g: &&cluster::Cluster| g.members.iter().map(|m| &m.app).collect::<std::collections::BTreeSet<_>>().len() >= 2;
    chosen.extend(groups.iter().filter(multi).filter(|g| args.redo || !exists(g)).take(args.top));
    if chosen.is_empty() {
        return Err("nothing to draft: pass --group or --top".into());
    }
    let index = Index::new(&args.data);
    let located: Vec<_> = apps.iter().map(|a| scan::locate(&args.root, a, scan::DEFAULT_REVS)).collect();
    let workdir = std::env::temp_dir().join("craft-parity-claude");
    fs::create_dir_all(&workdir).map_err(|e| format!("{}: {e}", workdir.display()))?;
    let cli = classify::ClaudeCli { model: args.model.clone(), effort: args.effort.clone(), workdir };
    for g in chosen {
        let key = draft::key(g).unwrap_or_default();
        if !args.redo && !args.verify_only && exists(g) && !args.group.is_empty() {
            eprintln!("{key}: already drafted (use --redo)");
            continue;
        }
        let file = dir.join(format!("{key}.json"));
        let drafted = if args.verify_only {
            fs::read_to_string(&file)
                .map_err(|e| format!("{}: {e}", file.display()))
                .and_then(|t| serde_json::from_str::<draft::Draft>(&t).map_err(|e| e.to_string()))
        } else {
            eprintln!("{key}: drafting ({} records, fixed in {})", g.members.len(), g.fixed_in.join(", "));
            draft::draft(&cli, g, &index, &apps, &args.root, &located, args.attempts)
        };
        let drafted = drafted.and_then(|mut d| {
            if d.proven {
                eprintln!("{key}: checking the apps that read missing");
                draft::verify(&cli, &mut d, &index, &apps, &args.root, &located)?;
            }
            Ok(d)
        });
        match drafted {
            Ok(d) => {
                let text = serde_json::to_string_pretty(&d).map_err(|e| e.to_string())? + "\n";
                craft_parity::index::write_if_changed(&dir.join(format!("{key}.json")), text.as_bytes())?;
                let missing: Vec<&String> = d.matrix.iter().filter(|(_, s)| **s == scan::Status::Missing).map(|(a, _)| a).collect();
                eprintln!(
                    "{key}: {} after {} attempt(s); proof {}/{}{}",
                    if d.proven { "PROVEN" } else { "not proven" },
                    d.attempts,
                    d.proof.iter().filter(|p| p.ok).count(),
                    d.proof.len(),
                    if d.proven {
                        format!("; missing in {}", missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                    } else {
                        String::new()
                    }
                );
            }
            Err(e) => eprintln!("{key}: {e}"),
        }
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
        Format::Html => {
            let groups: Vec<cluster::Cluster> = match &args.data {
                Some(data) => {
                    let path = data.join("llm").join("candidates.json");
                    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
                    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?
                }
                None => Vec::new(),
            };
            let mut drafts: Vec<draft::Draft> = Vec::new();
            if let Some(data) = &args.data
                && let Ok(entries) = fs::read_dir(data.join("llm").join("drafts"))
            {
                for e in entries.flatten() {
                    let text = fs::read_to_string(e.path()).map_err(|err| format!("{}: {err}", e.path().display()))?;
                    drafts.push(serde_json::from_str(&text).map_err(|err| format!("{}: {err}", e.path().display()))?);
                }
            }
            html::html(&report, concat!(env!("CARGO_PKG_REPOSITORY"), "/blob/main"), &groups, &drafts)
        }
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
        Command::Classify(args) => run_classify(args),
        Command::Cluster(args) => run_cluster(args),
        Command::Draft(args) => run_draft(args),
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
