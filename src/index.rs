//! The issue and pull request index: one JSONL file per kind and repo, sorted by number, so a sync
//! that changed nothing writes nothing and a diff shows exactly which records moved.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// How much of a body is kept: enough to describe a bug, small enough to keep the index light.
pub const BODY_EXCERPT_CHARS: usize = 2000;
/// Changed paths kept per pull request.
pub const MAX_PATHS: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueRef {
    /// `owner/name` of the repo the issue lives in; usually the PR's own repo.
    pub repo: String,
    pub number: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    /// `OPEN`, `CLOSED` or `MERGED`.
    pub state: String,
    pub draft: bool,
    pub author: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub closed_at: Option<String>,
    pub merge_commit: Option<String>,
    pub base_ref: String,
    pub head_ref: String,
    pub head_commit: Option<String>,
    /// Issues this PR closes ("Fixes #N" and linked issues).
    pub closes: Vec<IssueRef>,
    /// The first [`MAX_PATHS`] changed paths.
    pub paths: Vec<String>,
    pub paths_total: u64,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    /// `OPEN` or `CLOSED`.
    pub state: String,
    /// `COMPLETED`, `NOT_PLANNED`, `DUPLICATE`, `REOPENED`, or none while open.
    pub state_reason: Option<String>,
    pub author: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub body: String,
}

/// Where each repo's sync stopped.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoState {
    /// Newest `updatedAt` seen for pull requests and issues.
    pub prs_updated: Option<String>,
    pub issues_updated: Option<String>,
    /// When the last full (not incremental) sync finished.
    pub full_sync: Option<String>,
}

pub fn excerpt(body: &str) -> String {
    let body = body.replace("\r\n", "\n");
    let body = body.trim();
    match body.char_indices().nth(BODY_EXCERPT_CHARS) {
        Some((i, _)) => format!("{}…", &body[..i]),
        None => body.to_string(),
    }
}

/// The index on disk under one root folder.
pub struct Index {
    root: PathBuf,
}

impl Index {
    pub fn new(root: impl Into<PathBuf>) -> Index {
        Index { root: root.into() }
    }

    fn file(&self, repo: &str, kind: &str) -> PathBuf {
        self.root.join("index").join(repo).join(format!("{kind}.jsonl"))
    }

    fn state_file(&self) -> PathBuf {
        self.root.join("index").join("state.json")
    }

    pub fn prs(&self, repo: &str) -> Result<BTreeMap<u64, PullRequest>, String> {
        Ok(read_jsonl::<PullRequest>(&self.file(repo, "prs"))?.into_iter().map(|p| (p.number, p)).collect())
    }

    pub fn issues(&self, repo: &str) -> Result<BTreeMap<u64, Issue>, String> {
        Ok(read_jsonl::<Issue>(&self.file(repo, "issues"))?.into_iter().map(|i| (i.number, i)).collect())
    }

    pub fn write_prs(&self, repo: &str, prs: &BTreeMap<u64, PullRequest>) -> Result<bool, String> {
        write_jsonl(&self.file(repo, "prs"), prs.values())
    }

    pub fn write_issues(&self, repo: &str, issues: &BTreeMap<u64, Issue>) -> Result<bool, String> {
        write_jsonl(&self.file(repo, "issues"), issues.values())
    }

    pub fn state(&self) -> Result<BTreeMap<String, RepoState>, String> {
        let path = self.state_file();
        match fs::read_to_string(&path) {
            Ok(text) => serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
            Err(e) => Err(format!("{}: {e}", path.display())),
        }
    }

    pub fn write_state(&self, state: &BTreeMap<String, RepoState>) -> Result<bool, String> {
        let text = serde_json::to_string_pretty(state).map_err(|e| e.to_string())? + "\n";
        write_if_changed(&self.state_file(), text.as_bytes())
    }
}

pub fn read_jsonl<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>, String> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(n, line)| serde_json::from_str(line).map_err(|e| format!("{}:{}: {e}", path.display(), n + 1)))
        .collect()
}

/// Writes one JSON object per line; returns whether the file changed.
pub fn write_jsonl<'a, T: Serialize + 'a>(path: &Path, items: impl IntoIterator<Item = &'a T>) -> Result<bool, String> {
    let mut out = Vec::new();
    for item in items {
        serde_json::to_writer(&mut out, item).map_err(|e| e.to_string())?;
        out.push(b'\n');
    }
    write_if_changed(path, &out)
}

/// Writes through a temporary file and a rename, so an interrupted run never leaves half a file.
pub fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    if fs::read(path).is_ok_and(|old| old == bytes) {
        return Ok(false);
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let tmp = path.with_extension("tmp");
    let mut file = fs::File::create(&tmp).map_err(|e| format!("{}: {e}", tmp.display()))?;
    file.write_all(bytes).and_then(|_| file.sync_all()).map_err(|e| format!("{}: {e}", tmp.display()))?;
    fs::rename(&tmp, path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(true)
}
