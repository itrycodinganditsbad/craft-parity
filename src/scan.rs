//! Runs every fingerprint against every app.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::fix::{App, Check, Fix};
use crate::git;

/// Where one app stands on one fix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "detail", rename_all = "snake_case")]
pub enum Status {
    Fixed,
    Missing,
    /// `applies_if` failed: the app does not have the code the fix is about.
    NotApplicable,
    /// The fingerprint names a reason this app does not need the fix.
    Exempt(String),
    /// The app was not scanned (no checkout, or the git call failed).
    Unknown(String),
}

/// One scanned app: the commit read, or why none was.
#[derive(Debug, Clone, Serialize)]
pub struct AppScan {
    pub app: App,
    /// The ref that was read and its commit id.
    pub commit: Result<(String, String), String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FixRow {
    pub fix: Fix,
    /// One status per app, in app order.
    pub statuses: Vec<Status>,
    /// `known_fixed` apps that read as missing: the fingerprint no longer matches their fix.
    pub stale: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub apps: Vec<AppScan>,
    pub rows: Vec<FixRow>,
}

impl Report {
    pub fn is_stale(&self) -> bool {
        self.rows.iter().any(|r| !r.stale.is_empty())
    }
}

/// The ref read when the caller names none: the upstream default branch, so local feature
/// branches do not change the answer.
pub const DEFAULT_REVS: &[&str] = &["origin/HEAD", "origin/main", "origin/master", "HEAD"];

pub fn locate(root: &Path, app: &App, revs: &[&str]) -> AppScan {
    let repo = root.join(&app.name);
    let commit = if !repo.join(".git").exists() {
        Err(format!("no checkout at {}", repo.display()))
    } else {
        match git::resolve_first(&repo, revs) {
            Ok(Some(found)) => Ok(found),
            Ok(None) => Err(format!("none of {} names a commit", revs.join(", "))),
            Err(e) => Err(e),
        }
    };
    AppScan { app: app.clone(), commit }
}

fn all_pass(repo: &Path, commit: &str, checks: &[Check]) -> Result<bool, String> {
    for check in checks {
        if !git::passes(repo, commit, check)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn any_pass(repo: &Path, commit: &str, checks: &[Check]) -> Result<bool, String> {
    for check in checks {
        if git::passes(repo, commit, check)? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn status(root: &Path, scan: &AppScan, fix: &Fix) -> Status {
    if let Some(reason) = fix.exempt.get(&scan.app.name) {
        return Status::Exempt(reason.clone());
    }
    let (_, commit) = match &scan.commit {
        Ok(found) => found,
        Err(e) => return Status::Unknown(e.clone()),
    };
    let repo = root.join(&scan.app.name);
    let result = all_pass(&repo, commit, &fix.applies_if).and_then(|applies| {
        if !applies {
            return Ok(Status::NotApplicable);
        }
        Ok(if all_pass(&repo, commit, &fix.fixed_if)? || any_pass(&repo, commit, &fix.fixed_if_any)? { Status::Fixed } else { Status::Missing })
    });
    result.unwrap_or_else(Status::Unknown)
}

pub fn scan(root: &Path, apps: &[AppScan], fixes: &[Fix]) -> Report {
    let rows = fixes
        .iter()
        .map(|fix| {
            let statuses: Vec<Status> = apps.iter().map(|a| status(root, a, fix)).collect();
            let stale = apps
                .iter()
                .zip(&statuses)
                .filter(|(a, s)| fix.known_fixed.contains(&a.app.name) && !matches!(s, Status::Fixed | Status::Unknown(_)))
                .map(|(a, _)| a.app.name.clone())
                .collect();
            FixRow { fix: fix.clone(), statuses, stale }
        })
        .collect();
    Report { apps: apps.to_vec(), rows }
}
