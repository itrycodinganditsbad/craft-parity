//! Fix fingerprints (`fixes/*.toml`) and the app list (`apps.toml`).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// One Craft app repository.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct App {
    /// Short name, also the checkout folder under `--root`.
    pub name: String,
    /// `owner/repo` on GitHub.
    pub repo: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AppsFile {
    app: Vec<App>,
}

pub fn load_apps(path: &Path) -> Result<Vec<App>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let file: AppsFile = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(file.app)
}

/// One test against an app's tree at the scanned commit. Exactly one of `path`, `grep` or
/// `lockfile_crate` is set.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    /// A file matching this glob exists (git pathspec glob: `*` stays in one folder, `**` crosses).
    pub path: Option<String>,
    /// An extended regex matches a line in some file.
    pub grep: Option<String>,
    /// Globs that limit `grep` to some files. Empty means the whole tree.
    #[serde(default, rename = "in")]
    pub within: Vec<String>,
    #[serde(default)]
    pub ignore_case: bool,
    /// `Cargo.lock` at the repo root lists a package with this name.
    pub lockfile_crate: Option<String>,
}

impl Check {
    fn validate(&self) -> Result<(), String> {
        let set = [self.path.is_some(), self.grep.is_some(), self.lockfile_crate.is_some()].iter().filter(|s| **s).count();
        if set != 1 {
            return Err("a check needs exactly one of `path`, `grep` or `lockfile_crate`".into());
        }
        if (!self.within.is_empty() || self.ignore_case) && self.grep.is_none() {
            return Err("`in` and `ignore_case` only go with `grep`".into());
        }
        Ok(())
    }
}

/// A fix that landed in at least one app and may be missing from others.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fix {
    /// The file stem; not written in the file.
    #[serde(skip_deserializing)]
    pub id: String,
    pub title: String,
    /// What goes wrong without the fix, in a sentence or two.
    pub why: String,
    /// Where the fix landed, e.g. `storytold/pdfcraft#631`: the code to read before porting.
    pub references: Vec<String>,
    /// All must pass for the fix to matter to an app. Empty means it matters to every app.
    #[serde(default)]
    pub applies_if: Vec<Check>,
    /// All must pass for an app to count as fixed.
    pub fixed_if: Vec<Check>,
    /// Apps known to carry the fix. If one of them reads as missing, the fingerprint is stale.
    pub known_fixed: Vec<String>,
    /// App name to the reason the fix does not apply to it.
    #[serde(default)]
    pub exempt: BTreeMap<String, String>,
}

/// Loads every `*.toml` in `dir`, sorted by id.
pub fn load_fixes(dir: &Path, apps: &[App]) -> Result<Vec<Fix>, String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut fixes = Vec::new();
    for entry in entries {
        let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if path.extension().is_none_or(|ext| ext != "toml") {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut fix: Fix = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        fix.id = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
        validate(&fix, apps).map_err(|e| format!("{}: {e}", path.display()))?;
        fixes.push(fix);
    }
    fixes.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(fixes)
}

fn validate(fix: &Fix, apps: &[App]) -> Result<(), String> {
    if fix.fixed_if.is_empty() {
        return Err("`fixed_if` needs at least one check".into());
    }
    if fix.known_fixed.is_empty() {
        return Err("`known_fixed` needs at least one app, so a stale fingerprint gets caught".into());
    }
    for check in fix.applies_if.iter().chain(&fix.fixed_if) {
        check.validate()?;
    }
    for name in fix.known_fixed.iter().chain(fix.exempt.keys()) {
        if !apps.iter().any(|a| &a.name == name) {
            return Err(format!("unknown app `{name}`"));
        }
    }
    if let Some(name) = fix.known_fixed.iter().find(|n| fix.exempt.contains_key(*n)) {
        return Err(format!("`{name}` is both known_fixed and exempt"));
    }
    Ok(())
}
