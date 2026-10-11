//! Drafts a fingerprint for a group of the same bug fixed in several apps, then proves it on the
//! fixes themselves: every fix PR in the group must read missing on its merge commit's parent and
//! fixed on its merge commit. A draft that fails gets the failure back and another try.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::classify::{ClaudeCli, Usage, clip};
use crate::cluster::{Cluster, MemberState};
use crate::fix::{App, Check, Fix, Proof};
use crate::git;
use crate::index::Index;
use crate::scan::{self, AppScan, Status};

pub const PROMPT: &str = "\
You write fingerprints for craft-parity, a tool that checks which Craft apps (separate Rust repos \
with the same layout: apps/<name>/src/*.rs, crates/*, packaging/*) already carry a fix that landed \
in a sibling app. A fingerprint is a set of checks run against one commit of one repo:

- path: a file matching this glob exists (* stays in one folder, ** crosses folders).
- grep: an extended regex (git grep -E) matches a line in some file; `in` limits it to globs such \
as **/*.rs; ignore_case is optional.
- lockfile_crate: the root Cargo.lock lists a package with this exact name.

applies_if: all must pass for the fix to matter to an app (for example, the app has the control \
server or uses wgpu). Use [] when it matters to every app. fixed_if: all must pass for the app to \
count as fixed.

You get the same fix as several apps made it. Write checks that:
1. Pass in EVERY one of those apps after its fix, so look for what the diffs have in common: an \
API, a header name, a constant, a function, a crate, a file.
2. Fail BEFORE each fix, so the marker must be something the fix added, not something already \
there.
3. Would pass in another app that ported the fix in its own words: prefer API and protocol names \
over this app's own identifiers, and never use commit message wording, issue numbers or app names.
4. Stay narrow: limit grep with `in` to the file types or folders involved, and avoid patterns that \
would match comments or docs only.

Use one to three fixed_if checks. Keep regexes simple and valid for git grep -E (no lookaround, \
no \\d; use [0-9]). Write title and why in plain words: title names the fix as a short phrase, why \
says what goes wrong without it in one or two sentences.";

pub fn schema() -> Value {
    let check = json!({
        "type": "object",
        "properties": {
            "kind": { "type": "string", "enum": ["path", "grep", "lockfile_crate"] },
            "value": { "type": "string" },
            "in": { "type": "array", "items": { "type": "string" } },
            "ignore_case": { "type": "boolean" },
        },
        "required": ["kind", "value", "in", "ignore_case"],
        "additionalProperties": false,
    });
    json!({
        "type": "object",
        "properties": {
            "title": { "type": "string" },
            "why": { "type": "string" },
            "applies_if": { "type": "array", "items": check },
            "fixed_if": { "type": "array", "items": check },
            "reasoning": { "type": "string" },
        },
        "required": ["title", "why", "applies_if", "fixed_if", "reasoning"],
        "additionalProperties": false,
    })
}

fn to_check(v: &Value) -> Result<Check, String> {
    let value = v["value"].as_str().ok_or("check without value")?.to_string();
    let mut c = Check::default();
    match v["kind"].as_str() {
        Some("path") => c.path = Some(value),
        Some("grep") => {
            c.grep = Some(value);
            c.within = v["in"].as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default();
            c.ignore_case = v["ignore_case"].as_bool().unwrap_or(false);
        }
        Some("lockfile_crate") => c.lockfile_crate = Some(value),
        other => return Err(format!("unknown check kind {other:?}")),
    }
    Ok(c)
}

/// One fix PR from the group, located in its checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPr {
    pub app: String,
    pub repo: String,
    pub pr: u64,
    pub merge: String,
    pub parent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResult {
    pub app: String,
    pub pr: u64,
    pub before: Status,
    pub after: Status,
    pub ok: bool,
}

/// A stored draft: the fingerprint, how it did on its proof and what it says about every app now.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub key: String,
    pub area: String,
    pub symptom: String,
    pub attempts: u32,
    pub proven: bool,
    pub toml: String,
    pub reasoning: String,
    pub proof: Vec<ProofResult>,
    /// App name to its status on its default branch; empty unless proven.
    pub matrix: BTreeMap<String, Status>,
    /// For each app that read missing: what Claude found in its code.
    #[serde(default)]
    pub verified: BTreeMap<String, Verdict>,
    pub usage: Usage,
}

/// A stable name for a group: its first merged fix.
pub fn key(group: &Cluster) -> Option<String> {
    group.members.iter().find(|m| m.state == MemberState::Fixed).map(|m| m.id.replace('/', "-"))
}

/// The group's merged fixes that can be found in local checkouts, at most one per app.
pub fn fix_prs(group: &Cluster, index: &Index, apps: &[App], root: &Path) -> Result<Vec<FixPr>, String> {
    let mut out: Vec<FixPr> = Vec::new();
    for m in group.members.iter().filter(|m| m.state == MemberState::Fixed) {
        if out.iter().any(|f| f.app == m.app) {
            continue;
        }
        let Some(app) = apps.iter().find(|a| a.name == m.app) else { continue };
        let Some(pr) = m.id.rsplit('/').next().and_then(|n| n.parse::<u64>().ok()) else { continue };
        let Some(merge) = index.prs(&app.name)?.get(&pr).and_then(|p| p.merge_commit.clone()) else { continue };
        let repo = root.join(&app.name);
        if !repo.join(".git").exists() {
            continue;
        }
        let parent = git::ensure_commit(&repo, &merge)?;
        out.push(FixPr { app: app.name.clone(), repo: app.repo.clone(), pr, merge, parent });
    }
    Ok(out)
}

fn render_prompt(group: &Cluster, fixes: &[FixPr], root: &Path, feedback: &[String]) -> Result<String, String> {
    let mut p = String::from("The bug, as reported in different apps:\n");
    for m in group.members.iter().take(12) {
        p.push_str(&format!("- {}: {}\n", m.app, m.symptom));
    }
    for f in fixes.iter().take(3) {
        let diff = git::diff(&root.join(&f.app), &f.merge)?;
        p.push_str(&format!("\n<fix app=\"{}\" pr=\"{}\">\n{}\n</fix>\n", f.app, f.pr, clip(&diff, 14_000)));
    }
    if fixes.len() > 3 {
        let more: Vec<String> = fixes[3..].iter().map(|f| format!("{} #{}", f.app, f.pr)).collect();
        p.push_str(&format!("\nThe fingerprint is also checked against these fixes, not shown: {}.\n", more.join(", ")));
    }
    for (i, f) in feedback.iter().enumerate() {
        p.push_str(&format!("\nAttempt {} failed its proof:\n{f}\nWrite a corrected fingerprint.\n", i + 1));
    }
    Ok(p)
}

fn at(root: &Path, app: &App, commit: &str, fix: &Fix) -> Status {
    scan::status(root, &AppScan { app: app.clone(), commit: Ok(("proof".into(), commit.into())) }, fix)
}

/// Checks `fix` on every fix PR: missing (or not applicable) before, fixed after.
pub fn prove(root: &Path, apps: &[App], fixes: &[FixPr], fix: &Fix) -> Vec<ProofResult> {
    fixes
        .iter()
        .filter_map(|f| {
            let app = apps.iter().find(|a| a.name == f.app)?;
            let before = at(root, app, &f.parent, fix);
            let after = at(root, app, &f.merge, fix);
            let ok = matches!(before, Status::Missing | Status::NotApplicable) && after == Status::Fixed;
            Some(ProofResult { app: f.app.clone(), pr: f.pr, before, after, ok })
        })
        .collect()
}

fn feedback(results: &[ProofResult]) -> String {
    results
        .iter()
        .filter(|r| !r.ok)
        .map(|r| {
            let what = if r.after != Status::Fixed {
                format!("after the fix it reads {:?} (must be Fixed)", r.after)
            } else {
                "it already reads Fixed before the fix, so a marker was there all along".to_string()
            };
            format!("- {} PR #{}: {what}", r.app, r.pr)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The fingerprint as it would go in `fixes/<id>.toml` (the id is the file name, not a field).
pub fn to_toml(fix: &Fix) -> Result<String, String> {
    let mut value = toml::Table::try_from(fix).map_err(|e| e.to_string())?;
    value.remove("id");
    toml::to_string(&value).map_err(|e| e.to_string())
}

/// Drafts, proves and, once proven, scans a fingerprint for `group`.
pub fn draft(
    cli: &ClaudeCli,
    group: &Cluster,
    index: &Index,
    apps: &[App],
    root: &Path,
    located: &[AppScan],
    max_attempts: u32,
) -> Result<Draft, String> {
    let key = key(group).ok_or("group has no merged fix")?;
    let fixes = fix_prs(group, index, apps, root)?;
    if fixes.is_empty() {
        return Err(format!("{key}: no fix PR found in a local checkout"));
    }
    let mut feedbacks = Vec::new();
    let mut usage = Usage::default();
    let mut last: Option<(Fix, String, Vec<ProofResult>)> = None;
    for attempt in 1..=max_attempts {
        let prompt = render_prompt(group, &fixes, root, &feedbacks)?;
        let (v, u) = cli.call(PROMPT, &prompt, &schema())?;
        usage.calls += u.calls;
        usage.input_tokens += u.input_tokens;
        usage.output_tokens += u.output_tokens;
        usage.cost_usd += u.cost_usd;
        let checks = |k: &str| v[k].as_array().map(|a| a.iter().map(to_check).collect::<Result<Vec<_>, _>>()).unwrap_or(Ok(vec![]));
        let fix = Fix {
            id: key.clone(),
            title: v["title"].as_str().unwrap_or_default().trim().to_string(),
            why: v["why"].as_str().unwrap_or_default().trim().to_string(),
            references: fixes.iter().map(|f| format!("{}#{}", f.repo, f.pr)).collect(),
            applies_if: checks("applies_if")?,
            fixed_if: checks("fixed_if")?,
            fixed_if_any: Vec::new(),
            known_fixed: fixes.iter().map(|f| f.app.clone()).collect(),
            exempt: BTreeMap::new(),
            proof: fixes.iter().map(|f| Proof { app: f.app.clone(), pr: f.pr }).collect(),
        };
        if let Err(e) = crate::fix::validate(&fix, apps) {
            feedbacks.push(format!("- the fingerprint is invalid: {e}"));
            continue;
        }
        let results = prove(root, apps, &fixes, &fix);
        let reasoning = v["reasoning"].as_str().unwrap_or_default().to_string();
        if results.iter().all(|r| r.ok) {
            let matrix = located.iter().map(|a| (a.app.name.clone(), scan::status(root, a, &fix))).collect();
            return Ok(Draft {
                key,
                area: group.area.clone(),
                symptom: group.members[0].symptom.clone(),
                attempts: attempt,
                proven: true,
                toml: to_toml(&fix)?,
                reasoning,
                proof: results,
                matrix,
                verified: BTreeMap::new(),
                usage,
            });
        }
        feedbacks.push(feedback(&results));
        last = Some((fix, reasoning, results));
    }
    let (fix, reasoning, proof) = last.ok_or("no valid draft")?;
    Ok(Draft {
        key,
        area: group.area.clone(),
        symptom: group.members[0].symptom.clone(),
        attempts: max_attempts,
        proven: false,
        toml: to_toml(&fix)?,
        reasoning,
        proof,
        matrix: BTreeMap::new(),
        verified: BTreeMap::new(),
        usage,
    })
}

pub const VERIFY_PROMPT: &str = "\
You check one Craft app for a bug that sibling apps already fixed. You get the fix as one sibling \
made it, a description, and the relevant source files of the app being checked, at its current \
default branch. Decide:

- present: the app has the bug; the code does what the unfixed sibling did.
- fixed_differently: the app is protected some other way (for example authentication, a different \
check or a different design).
- not_applicable: the app lacks the feature the bug lives in (the file shown is unrelated code).
- unclear: the files shown are not enough to tell.

Quote the few lines your verdict rests on in evidence, with the file name. For fixed_differently, \
give one check that detects this app's protection: a grep (git grep -E regex, with `in` globs) or a \
path, specific enough that it would not match code without the protection. Otherwise alternative \
is null. Base the verdict only on the code shown.";

/// What Claude found in one app that read missing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verdict {
    /// `present`, `fixed_differently`, `not_applicable` or `unclear`.
    pub verdict: String,
    pub evidence: String,
    pub explanation: String,
    /// The fingerprint check this app's own fix would match, when it was accepted.
    pub alternative: Option<Check>,
    /// Why a proposed alternative was not used.
    pub alternative_rejected: Option<String>,
}

fn verify_schema() -> Value {
    let check = schema()["properties"]["fixed_if"]["items"].clone();
    json!({
        "type": "object",
        "properties": {
            "verdict": { "type": "string", "enum": ["present", "fixed_differently", "not_applicable", "unclear"] },
            "evidence": { "type": "string" },
            "explanation": { "type": "string" },
            "alternative": { "anyOf": [check, { "type": "null" }] },
        },
        "required": ["verdict", "evidence", "explanation", "alternative"],
        "additionalProperties": false,
    })
}

/// The files worth showing for `app`: the fix PRs' paths moved into this app, then files the
/// fingerprint's applies_if points at.
fn files_for(root: &Path, app: &App, commit: &str, fix: &Fix, fixes: &[FixPr], index: &Index) -> Result<Vec<(String, String)>, String> {
    let repo = root.join(&app.name);
    let mut paths: Vec<String> = Vec::new();
    for f in fixes {
        if let Some(pr) = index.prs(&f.app)?.get(&f.pr) {
            for p in &pr.paths {
                let moved = p.replace(&f.app, &app.name);
                if !paths.contains(&moved) && !moved.ends_with(".md") && !moved.contains("/tests") {
                    paths.push(moved);
                }
            }
        }
    }
    for check in &fix.applies_if {
        for p in git::matching_files(&repo, commit, check)? {
            if !paths.contains(&p) && !p.contains("/tests") {
                paths.push(p);
            }
        }
    }
    let mut out = Vec::new();
    for p in paths {
        if out.len() == 4 {
            break;
        }
        if let Some(text) = git::show(&repo, commit, &p)? {
            out.push((p, clip(&text, 10_000)));
        }
    }
    Ok(out)
}

/// Asks Claude to look at every app that reads missing, and folds what it finds back into the
/// draft: an accepted alternative check goes into `fixed_if_any`, a not-applicable app is exempted.
/// An alternative is accepted only if it passes on that app now and fails before every proven fix.
pub fn verify(cli: &ClaudeCli, d: &mut Draft, index: &Index, apps: &[App], root: &Path, located: &[AppScan]) -> Result<(), String> {
    let mut fix: Fix = toml::from_str(&d.toml).map_err(|e| e.to_string())?;
    fix.id = d.key.clone();
    let fixes: Vec<FixPr> = fix_prs_by_proof(&fix, index, apps, root)?;
    let reference = match fixes.first() {
        Some(f) => clip(&git::diff(&root.join(&f.app), &f.merge)?, 6_000),
        None => String::new(),
    };
    let missing: Vec<&AppScan> = located.iter().filter(|a| d.matrix.get(&a.app.name) == Some(&Status::Missing)).collect();
    for a in missing {
        let Ok((_, commit)) = &a.commit else { continue };
        let files = files_for(root, &a.app, commit, &fix, &fixes, index)?;
        let mut prompt = format!(
            "Bug: {}\n{}\n\nThe fix as a sibling made it:\n<diff>\n{reference}\n</diff>\n\nApp being checked: {}\n",
            fix.title, fix.why, a.app.name
        );
        if files.is_empty() {
            prompt.push_str("No related files were found in this app.\n");
        }
        for (p, text) in &files {
            prompt.push_str(&format!("\n<file path=\"{p}\">\n{text}\n</file>\n"));
        }
        let (v, u) = cli.call(VERIFY_PROMPT, &prompt, &verify_schema())?;
        d.usage.calls += u.calls;
        d.usage.input_tokens += u.input_tokens;
        d.usage.output_tokens += u.output_tokens;
        d.usage.cost_usd += u.cost_usd;
        let mut verdict = Verdict {
            verdict: v["verdict"].as_str().unwrap_or("unclear").to_string(),
            evidence: v["evidence"].as_str().unwrap_or_default().to_string(),
            explanation: v["explanation"].as_str().unwrap_or_default().to_string(),
            alternative: None,
            alternative_rejected: None,
        };
        match verdict.verdict.as_str() {
            "fixed_differently" if v["alternative"].is_object() => {
                let check = to_check(&v["alternative"])?;
                let repo = root.join(&a.app.name);
                let now = git::passes(&repo, commit, &check)?;
                let mut before_any = false;
                for f in &fixes {
                    before_any |= git::passes(&root.join(&f.app), &f.parent, &check)?;
                }
                if now && !before_any {
                    fix.fixed_if_any.push(check.clone());
                    verdict.alternative = Some(check);
                } else {
                    verdict.alternative_rejected =
                        Some(if now { "it also matches before a proven fix".into() } else { "it does not match this app".into() });
                }
            }
            "not_applicable" => {
                fix.exempt.insert(a.app.name.clone(), clip(&verdict.explanation, 160));
            }
            _ => {}
        }
        d.verified.insert(a.app.name.clone(), verdict);
    }
    // The proof must still hold with the alternatives added.
    let results = prove(root, apps, &fixes, &fix);
    if !results.iter().all(|r| r.ok) {
        return Err("an accepted alternative broke the proof".into());
    }
    d.toml = to_toml(&fix)?;
    d.matrix = located.iter().map(|a| (a.app.name.clone(), scan::status(root, a, &fix))).collect();
    Ok(())
}

fn fix_prs_by_proof(fix: &Fix, index: &Index, apps: &[App], root: &Path) -> Result<Vec<FixPr>, String> {
    let mut out = Vec::new();
    for p in &fix.proof {
        let Some(app) = apps.iter().find(|a| a.name == p.app) else { continue };
        let Some(merge) = index.prs(&app.name)?.get(&p.pr).and_then(|pr| pr.merge_commit.clone()) else { continue };
        let parent = git::ensure_commit(&root.join(&app.name), &merge)?;
        out.push(FixPr { app: app.name.clone(), repo: app.repo.clone(), pr: p.pr, merge, parent });
    }
    Ok(out)
}
