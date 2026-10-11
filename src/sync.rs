//! Brings the index up to date with GitHub, one repo at a time.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use crate::fix::App;
use crate::github::{self, GraphQl};
use crate::index::{self, Index, Issue, IssueRef, PullRequest, RepoState};

/// An incremental sync re-reads this much before its cursor, so an item updated while the last run
/// was paging is not missed.
pub const OVERLAP_SECS: i64 = 600;
const PR_PAGE: u64 = 50;
const ISSUE_PAGE: u64 = 100;

/// What one repo's sync did.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub prs_seen: usize,
    pub prs_changed: usize,
    pub issues_seen: usize,
    pub issues_changed: usize,
    pub removed: usize,
}

fn s(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
}

fn opt(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_string)
}

fn login(v: &Value) -> Option<String> {
    v.get("author").and_then(|a| a.get("login")).and_then(Value::as_str).map(str::to_string)
}

fn number(v: &Value) -> Result<u64, String> {
    v.get("number").and_then(Value::as_u64).ok_or_else(|| format!("node without a number: {v}"))
}

fn nodes<'a>(v: &'a Value, path: &[&str]) -> &'a [Value] {
    let mut v = v;
    for key in path {
        v = &v[*key];
    }
    v["nodes"].as_array().map(Vec::as_slice).unwrap_or_default()
}

pub fn parse_pr(node: &Value) -> Result<PullRequest, String> {
    let closes = nodes(node, &["closingIssuesReferences"])
        .iter()
        .map(|n| Ok(IssueRef { repo: n["repository"]["nameWithOwner"].as_str().unwrap_or_default().to_string(), number: number(n)? }))
        .collect::<Result<Vec<_>, String>>()?;
    let mut paths: Vec<String> = nodes(node, &["files"]).iter().filter_map(|f| f["path"].as_str().map(str::to_string)).collect();
    paths.truncate(index::MAX_PATHS);
    Ok(PullRequest {
        number: number(node)?,
        title: s(node, "title"),
        state: s(node, "state"),
        draft: node["isDraft"].as_bool().unwrap_or(false),
        author: login(node),
        created_at: s(node, "createdAt"),
        updated_at: s(node, "updatedAt"),
        merged_at: opt(node, "mergedAt"),
        closed_at: opt(node, "closedAt"),
        merge_commit: node["mergeCommit"]["oid"].as_str().map(str::to_string),
        base_ref: s(node, "baseRefName"),
        head_ref: s(node, "headRefName"),
        head_commit: opt(node, "headRefOid"),
        closes,
        paths,
        paths_total: node["files"]["totalCount"].as_u64().unwrap_or(0),
        body: index::excerpt(&s(node, "body")),
    })
}

pub fn parse_issue(node: &Value) -> Result<Issue, String> {
    Ok(Issue {
        number: number(node)?,
        title: s(node, "title"),
        state: s(node, "state"),
        state_reason: opt(node, "stateReason"),
        author: login(node),
        created_at: s(node, "createdAt"),
        updated_at: s(node, "updatedAt"),
        closed_at: opt(node, "closedAt"),
        body: index::excerpt(&s(node, "body")),
    })
}

/// Pages through one connection, newest update first, until a page reaches `stop_before` (or the
/// end when it is `None`). Calls `each` for every node.
fn page(
    gh: &dyn GraphQl,
    query: &str,
    connection: &str,
    repo: &str,
    first: u64,
    stop_before: Option<i64>,
    mut each: impl FnMut(&Value) -> Result<(), String>,
) -> Result<(), String> {
    let (owner, name) = repo.split_once('/').ok_or_else(|| format!("repo `{repo}` is not owner/name"))?;
    let mut after: Option<String> = None;
    loop {
        let data = gh.query(query, json!({ "owner": owner, "name": name, "after": after, "first": first }))?;
        let conn = &data["repository"][connection];
        if conn.is_null() {
            return Err(format!("{repo}: no repository or no access"));
        }
        let mut reached = false;
        for node in conn["nodes"].as_array().map(Vec::as_slice).unwrap_or_default() {
            each(node)?;
            if let Some(stop) = stop_before
                && parse_time(node["updatedAt"].as_str().unwrap_or_default()).is_some_and(|t| t < stop)
            {
                reached = true;
            }
        }
        if reached || conn["pageInfo"]["hasNextPage"].as_bool() != Some(true) {
            return Ok(());
        }
        after = conn["pageInfo"]["endCursor"].as_str().map(str::to_string);
    }
}

fn max_time(a: Option<String>, b: &str) -> Option<String> {
    match a {
        Some(a) if a.as_str() >= b => Some(a),
        _ if b.is_empty() => None,
        _ => Some(b.to_string()),
    }
}

/// Syncs one app's repo into `index`. `full` re-reads everything and drops records GitHub no longer
/// returns (deleted or transferred issues).
pub fn sync_repo(gh: &dyn GraphQl, index: &Index, app: &App, state: &mut RepoState, full: bool, now: &str) -> Result<Counts, String> {
    let mut counts = Counts::default();
    let back = |cursor: &Option<String>| if full { None } else { cursor.as_deref().and_then(parse_time).map(|t| t - OVERLAP_SECS) };

    let mut prs = index.prs(&app.name)?;
    let mut seen = BTreeSet::new();
    let mut newest = state.prs_updated.clone();
    page(gh, github::PRS, "pullRequests", &app.repo, PR_PAGE, back(&state.prs_updated), |node| {
        let pr = parse_pr(node)?;
        counts.prs_seen += 1;
        newest = max_time(newest.take(), &pr.updated_at);
        seen.insert(pr.number);
        if prs.get(&pr.number) != Some(&pr) {
            counts.prs_changed += 1;
            prs.insert(pr.number, pr);
        }
        Ok(())
    })?;
    if full {
        let before = prs.len();
        prs.retain(|n, _| seen.contains(n));
        counts.removed += before - prs.len();
    }
    index.write_prs(&app.name, &prs)?;
    state.prs_updated = newest;

    let mut issues = index.issues(&app.name)?;
    let mut seen = BTreeSet::new();
    let mut newest = state.issues_updated.clone();
    page(gh, github::ISSUES, "issues", &app.repo, ISSUE_PAGE, back(&state.issues_updated), |node| {
        let issue = parse_issue(node)?;
        counts.issues_seen += 1;
        newest = max_time(newest.take(), &issue.updated_at);
        seen.insert(issue.number);
        if issues.get(&issue.number) != Some(&issue) {
            counts.issues_changed += 1;
            issues.insert(issue.number, issue);
        }
        Ok(())
    })?;
    if full {
        let before = issues.len();
        issues.retain(|n, _| seen.contains(n));
        counts.removed += before - issues.len();
        state.full_sync = Some(now.to_string());
    }
    index.write_issues(&app.name, &issues)?;
    state.issues_updated = newest;
    Ok(counts)
}

/// Syncs every app, saving the cursor after each repo so a failure part way keeps what was done.
pub fn sync_all(gh: &dyn GraphQl, index: &Index, apps: &[App], full: bool) -> Result<BTreeMap<String, Counts>, String> {
    let now = format_time(SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0));
    let mut state = index.state()?;
    let mut all = BTreeMap::new();
    for app in apps {
        let entry = state.entry(app.name.clone()).or_default();
        let counts = sync_repo(gh, index, app, entry, full, &now).map_err(|e| format!("{}: {e}", app.name))?;
        index.write_state(&state)?;
        all.insert(app.name.clone(), counts);
    }
    Ok(all)
}

/// Seconds since the Unix epoch for GitHub's `YYYY-MM-DDTHH:MM:SSZ`.
pub fn parse_time(text: &str) -> Option<i64> {
    let b = text.as_bytes();
    if b.len() != 20 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' || b[13] != b':' || b[16] != b':' || b[19] != b'Z' {
        return None;
    }
    let n = |r: std::ops::Range<usize>| text.get(r)?.parse::<i64>().ok();
    let (y, m, d) = (n(0..4)?, n(5..7)?, n(8..10)?);
    let (hh, mm, ss) = (n(11..13)?, n(14..16)?, n(17..19)?);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || hh > 23 || mm > 59 || ss > 60 {
        return None;
    }
    // Days from civil, Howard Hinnant's algorithm.
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + hh * 3600 + mm * 60 + ss)
}

pub fn format_time(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}
