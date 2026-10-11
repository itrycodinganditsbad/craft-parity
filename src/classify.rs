//! Asks Claude what each fix, open PR and open issue is about, in batches, and keeps the answers
//! next to the index. Each answer is keyed by the record and a hash of exactly what was sent, so a
//! record is only sent again when its text, the prompt or the model changes.

use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::fix::App;
use crate::index::{self, Index};

/// Bump when the prompt or schema changes in a way that should re-classify everything.
pub const PROMPT_VERSION: u32 = 1;

pub const SYSTEM_PROMPT: &str = "\
You classify pull requests and issues from the Craft App Suite: open-source, clean-room desktop apps \
in pure Rust that each copy a well-known commercial app. PhotoCraft (Photoshop), LightCraft \
(Lightroom), FilmCraft (Premiere), EffectCraft (After Effects), DesignCraft (InDesign), VectorCraft \
(Illustrator), PdfCraft (Acrobat), WordCraft (Word), DeckCraft (PowerPoint), GridCraft (Excel), \
CADCraft (AutoCAD), SoundCraft (audio editing) and the Craft launcher are egui + wgpu apps with the \
same layout (apps/<name>/src/main.rs, crates/ui-egui, packaging/macos|windows|linux, a control \
server for automation). ArtCraft and ArtCraftX are Tauri apps with a web front end. The repos share \
conventions but never code, so a bug fixed in one app often still exists in the others.

For each record, decide:
- kind: what the change or report is. bug_fix for a merged fix or a bug report; feature for new \
behaviour; refactor, docs, build_ci, release, other.
- scope: could the same bug or gap plausibly exist in OTHER Craft apps? suite_wide when the cause \
lives in code every app has (window and GPU setup, fonts and text shaping, i18n, logging, \
packaging and installers, file dialogs, theme, input, the control server, startup, crash handling, \
platform quirks). app_specific when it is about this app's own domain (PDF parsing, layer blending, \
timeline editing, spreadsheet formulas). unclear when you can't tell.
- area and platforms: pick from the lists.
- symptom: one plain sentence a user would recognise, written WITHOUT the app's name, so the same \
bug in two apps gets nearly the same sentence. Example: \"On Windows desktops with two GPUs the \
window stays black or the display driver resets.\"
- root_cause: one sentence, also without the app's name. Empty if unknown.
- fixed_markers: up to 5 literal strings (API names, function names, file paths, crate names) whose \
presence in a repo would show the fix is in. Only for merged fixes; empty otherwise.
- bug_markers: up to 5 literal strings whose presence suggests the bug is present. Empty if none.
- confidence: 0 to 1, how sure you are of scope.

Return exactly one result per record, with its id unchanged. Base everything on the text given; \
do not invent details.";

const AREAS: &[&str] = &[
    "gpu_rendering",
    "windowing_input",
    "text_fonts_i18n",
    "packaging_install",
    "logging_diagnostics",
    "file_io",
    "performance",
    "ui_theme",
    "accessibility",
    "automation_control",
    "security",
    "build_ci",
    "app_domain",
    "other",
];
const KINDS: &[&str] = &["bug_fix", "feature", "refactor", "docs", "build_ci", "release", "other"];
const SCOPES: &[&str] = &["app_specific", "suite_wide", "unclear"];
const PLATFORMS: &[&str] = &["windows", "macos", "linux", "web", "freebsd", "all"];

/// What the model returns for one record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Labels {
    pub kind: String,
    pub scope: String,
    pub area: String,
    pub platforms: Vec<String>,
    pub symptom: String,
    pub root_cause: String,
    pub fixed_markers: Vec<String>,
    pub bug_markers: Vec<String>,
    pub confidence: f64,
}

/// One stored answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Classified {
    /// `<app>/pr/<n>` or `<app>/issue/<n>`.
    pub id: String,
    /// Hash of the rendered record, the prompt version and the model.
    pub input: String,
    pub model: String,
    pub labels: Labels,
}

/// A record ready to send.
#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub text: String,
    pub hash: String,
    /// Sort key: merged fixes first, then open PRs, then issues; newest first within each.
    pub rank: (u8, std::cmp::Reverse<String>),
}

pub fn schema() -> Value {
    let strings = json!({ "type": "array", "items": { "type": "string" } });
    json!({
        "type": "object",
        "properties": { "results": { "type": "array", "items": {
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "kind": { "type": "string", "enum": KINDS },
                "scope": { "type": "string", "enum": SCOPES },
                "area": { "type": "string", "enum": AREAS },
                "platforms": { "type": "array", "items": { "type": "string", "enum": PLATFORMS } },
                "symptom": { "type": "string" },
                "root_cause": { "type": "string" },
                "fixed_markers": strings,
                "bug_markers": strings,
                "confidence": { "type": "number" },
            },
            "required": ["id", "kind", "scope", "area", "platforms", "symptom", "root_cause", "fixed_markers", "bug_markers", "confidence"],
            "additionalProperties": false,
        }}},
        "required": ["results"],
        "additionalProperties": false,
    })
}

/// FNV-1a, 64 bit: a stable change detector, not a security boundary.
fn hash(parts: &[&str]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for part in parts {
        for b in part.bytes().chain([0xff]) {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("{h:016x}")
}

pub fn clip(text: &str, chars: usize) -> String {
    match text.char_indices().nth(chars) {
        Some((i, _)) => format!("{}…", &text[..i]),
        None => text.to_string(),
    }
}

/// Every record worth classifying in one app, rendered as text.
pub fn items(index: &Index, app: &App, model: &str) -> Result<Vec<Item>, String> {
    let prs = index.prs(&app.name)?;
    let issues = index.issues(&app.name)?;
    let mut fixed_issues = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    let mut push = |id: String, text: String, tier: u8, when: &str| {
        let hash = hash(&[&text, &PROMPT_VERSION.to_string(), model]);
        out.push(Item { id, text, hash, rank: (tier, std::cmp::Reverse(when.to_string())) });
    };

    for pr in prs.values() {
        let merged = pr.state == "MERGED";
        if !merged && !(pr.state == "OPEN" && !pr.draft) {
            continue;
        }
        let mut text = format!(
            "{} PR #{} ({})\nTitle: {}\nChanged paths ({} total): {}\nBody:\n{}\n",
            app.name,
            pr.number,
            if merged { "merged" } else { "open" },
            pr.title,
            pr.paths_total,
            pr.paths.iter().take(30).cloned().collect::<Vec<_>>().join(", "),
            clip(&pr.body, 1200)
        );
        for r in &pr.closes {
            if r.repo != app.repo {
                continue;
            }
            if merged {
                fixed_issues.insert(r.number);
            }
            if let Some(issue) = issues.get(&r.number) {
                text.push_str(&format!("Closes issue #{}: {}\n{}\n", issue.number, issue.title, clip(&issue.body, 600)));
            }
        }
        let tier = if merged { 0 } else { 1 };
        push(format!("{}/pr/{}", app.name, pr.number), text, tier, pr.merged_at.as_deref().unwrap_or(&pr.updated_at));
    }
    for issue in issues.values() {
        if issue.state != "OPEN" || fixed_issues.contains(&issue.number) {
            continue;
        }
        let text = format!("{} issue #{} (open)\nTitle: {}\nBody:\n{}\n", app.name, issue.number, issue.title, clip(&issue.body, 1200));
        push(format!("{}/issue/{}", app.name, issue.number), text, 2, &issue.created_at);
    }
    Ok(out)
}

/// Anything that can label a batch. The CLI backend runs `claude -p`; tests use a fake.
pub trait Classifier: Sync {
    fn model(&self) -> &str;
    /// Labels keyed by item id. Items missing from the answer are retried on a later run.
    fn classify(&self, batch: &[Item]) -> Result<(BTreeMap<String, Labels>, Usage), String>;
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub calls: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    /// List-price estimate reported by the CLI; on a subscription nothing is billed.
    pub cost_usd: f64,
}

impl Usage {
    fn add(&mut self, other: &Usage) {
        self.calls += other.calls;
        self.input_tokens += other.input_tokens;
        self.output_tokens += other.output_tokens;
        self.cost_usd += other.cost_usd;
    }
}

fn render_batch(batch: &[Item]) -> String {
    let mut prompt = format!("Classify these {} records.\n", batch.len());
    for item in batch {
        prompt.push_str(&format!("\n<record id=\"{}\">\n{}</record>\n", item.id, item.text));
    }
    prompt
}

/// Parses the model's `{"results": [...]}`, keeping only well-formed answers for ids in the batch.
pub fn parse_results(value: &Value, batch: &[Item]) -> BTreeMap<String, Labels> {
    let mut out = BTreeMap::new();
    for r in value["results"].as_array().map(Vec::as_slice).unwrap_or_default() {
        let Some(id) = r["id"].as_str() else { continue };
        if !batch.iter().any(|i| i.id == id) {
            continue;
        }
        let mut r = r.clone();
        if let Some(obj) = r.as_object_mut() {
            obj.remove("id");
        }
        if let Ok(labels) = serde_json::from_value::<Labels>(r)
            && KINDS.contains(&labels.kind.as_str())
            && SCOPES.contains(&labels.scope.as_str())
            && AREAS.contains(&labels.area.as_str())
        {
            out.insert(id.to_string(), labels);
        }
    }
    out
}

/// Runs `claude -p` on the logged-in subscription: no API key, no tools, no project settings.
pub struct ClaudeCli {
    pub model: String,
    pub effort: String,
    /// An empty folder to run in, so no CLAUDE.md is picked up.
    pub workdir: PathBuf,
}

impl Classifier for ClaudeCli {
    fn model(&self) -> &str {
        &self.model
    }

    fn classify(&self, batch: &[Item]) -> Result<(BTreeMap<String, Labels>, Usage), String> {
        let (value, usage) = self.call(SYSTEM_PROMPT, &render_batch(batch), &schema())?;
        Ok((parse_results(&value, batch), usage))
    }
}

impl ClaudeCli {
    /// One `claude -p` call that must answer with JSON matching `schema`.
    pub fn call(&self, system: &str, prompt: &str, schema: &Value) -> Result<(Value, Usage), String> {
        let mut child = Command::new("claude")
            .current_dir(&self.workdir)
            .env_remove("ANTHROPIC_API_KEY")
            .env_remove("ANTHROPIC_AUTH_TOKEN")
            .args(["-p", "--safe-mode", "--tools", "", "--strict-mcp-config", "--no-session-persistence", "--max-turns", "4"])
            .args(["--model", &self.model, "--effort", &self.effort, "--output-format", "json"])
            .args(["--system-prompt", system, "--json-schema", &schema.to_string()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("running claude: {e}"))?;
        child.stdin.take().ok_or("no stdin")?.write_all(prompt.as_bytes()).map_err(|e| format!("writing to claude: {e}"))?;
        let out = child.wait_with_output().map_err(|e| format!("running claude: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let envelope: Value = serde_json::from_str(&stdout).map_err(|_| {
            format!(
                "claude exited {:?} without JSON: {} {}",
                out.status.code(),
                clip(stdout.trim(), 400),
                clip(String::from_utf8_lossy(&out.stderr).trim(), 400)
            )
        })?;
        if !out.status.success() || envelope["is_error"].as_bool() == Some(true) || envelope["subtype"] != "success" {
            return Err(format!(
                "claude failed ({}, exit {:?}): {}",
                envelope["subtype"].as_str().unwrap_or("?"),
                out.status.code(),
                clip(envelope["result"].as_str().unwrap_or_default(), 400)
            ));
        }
        let u = &envelope["usage"];
        let tokens = |k: &str| u[k].as_u64().unwrap_or(0);
        let usage = Usage {
            calls: 1,
            input_tokens: tokens("input_tokens") + tokens("cache_creation_input_tokens") + tokens("cache_read_input_tokens"),
            output_tokens: tokens("output_tokens"),
            cost_usd: envelope["total_cost_usd"].as_f64().unwrap_or(0.0),
        };
        Ok((envelope["structured_output"].clone(), usage))
    }
}

/// Where answers live: `llm/classified/<app>.jsonl`, sorted by id.
pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn new(data: impl Into<PathBuf>) -> Store {
        Store { root: data.into().join("llm") }
    }

    fn file(&self, app: &str) -> PathBuf {
        self.root.join("classified").join(format!("{app}.jsonl"))
    }

    pub fn load(&self, app: &str) -> Result<BTreeMap<String, Classified>, String> {
        Ok(index::read_jsonl::<Classified>(&self.file(app))?.into_iter().map(|c| (c.id.clone(), c)).collect())
    }

    pub fn save(&self, app: &str, all: &BTreeMap<String, Classified>) -> Result<bool, String> {
        index::write_jsonl(&self.file(app), all.values())
    }

    fn log_usage(&self, usage: &Usage, model: &str, items: usize) -> Result<(), String> {
        let path = self.root.join("usage.jsonl");
        std::fs::create_dir_all(&self.root).map_err(|e| e.to_string())?;
        let line = json!({ "model": model, "items": items, "usage": usage }).to_string() + "\n";
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        f.write_all(line.as_bytes()).map_err(|e| format!("{}: {e}", path.display()))
    }
}

pub struct Options {
    pub batch_size: usize,
    pub jobs: usize,
    /// Stop after this many calls; `None` for no cap.
    pub max_calls: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Outcome {
    pub pending: usize,
    pub classified: usize,
    pub calls: usize,
    pub usage: Usage,
    /// Set when a call failed; the run stopped there and can be resumed.
    pub stopped: Option<String>,
}

/// Classifies every record whose stored answer is missing or out of date. Saves after each batch,
/// so an interrupted run loses at most the batches in flight.
pub fn run(classifier: &dyn Classifier, index: &Index, store: &Store, apps: &[App], opts: &Options) -> Result<Outcome, String> {
    let model = classifier.model().to_string();
    let mut stored: BTreeMap<String, BTreeMap<String, Classified>> = BTreeMap::new();
    let mut todo: Vec<(String, Item)> = Vec::new();
    for app in apps {
        let have = store.load(&app.name)?;
        for item in items(index, app, &model)? {
            if have.get(&item.id).is_none_or(|c| c.input != item.hash) {
                todo.push((app.name.clone(), item));
            }
        }
        stored.insert(app.name.clone(), have);
    }
    // Mix apps within a rank so an early stop still covers every app.
    todo.sort_by(|a, b| a.1.rank.cmp(&b.1.rank).then_with(|| a.1.id.cmp(&b.1.id)));
    let outcome = Outcome { pending: todo.len(), ..Default::default() };

    let mut batches: Vec<Vec<(String, Item)>> = todo.chunks(opts.batch_size.max(1)).map(<[_]>::to_vec).collect();
    if let Some(max) = opts.max_calls {
        batches.truncate(max);
    }
    let queue = Mutex::new(batches.into_iter());
    let state = Mutex::new((stored, outcome));

    std::thread::scope(|s| {
        for _ in 0..opts.jobs.max(1) {
            s.spawn(|| {
                loop {
                    if state.lock().map(|g| g.1.stopped.is_some()).unwrap_or(true) {
                        return;
                    }
                    let Some(batch) = queue.lock().ok().and_then(|mut q| q.next()) else { return };
                    let items: Vec<Item> = batch.iter().map(|(_, i)| i.clone()).collect();
                    let result = classifier.classify(&items);
                    let Ok(mut guard) = state.lock() else { return };
                    let (stored, outcome) = &mut *guard;
                    match result {
                        Err(e) => {
                            outcome.stopped.get_or_insert(e);
                        }
                        Ok((labels, usage)) => {
                            outcome.calls += 1;
                            outcome.usage.add(&usage);
                            let mut touched = std::collections::BTreeSet::new();
                            for (app, item) in &batch {
                                if let Some(l) = labels.get(&item.id) {
                                    stored.entry(app.clone()).or_default().insert(
                                        item.id.clone(),
                                        Classified { id: item.id.clone(), input: item.hash.clone(), model: model.clone(), labels: l.clone() },
                                    );
                                    outcome.classified += 1;
                                    touched.insert(app.clone());
                                }
                            }
                            for app in touched {
                                if let Err(e) = store.save(&app, &stored[&app]) {
                                    outcome.stopped.get_or_insert(e);
                                }
                            }
                            if let Err(e) = store.log_usage(&usage, &model, labels.len()) {
                                outcome.stopped.get_or_insert(e);
                            }
                            eprintln!(
                                "classified {}/{} this run ({} calls, ~{}k tokens in)",
                                outcome.classified,
                                outcome.pending,
                                outcome.calls,
                                outcome.usage.input_tokens / 1000
                            );
                        }
                    }
                }
            });
        }
    });
    let (_, outcome) = state.into_inner().map_err(|_| "a worker panicked".to_string())?;
    Ok(outcome)
}
