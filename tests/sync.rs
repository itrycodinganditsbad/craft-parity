//! The sync against a fake GitHub that serves pages from memory.

use std::cell::RefCell;
use std::fs;

use serde_json::{Value, json};

use craft_parity::fix::App;
use craft_parity::github::GraphQl;
use craft_parity::index::{Index, RepoState};
use craft_parity::sync::{self, format_time, parse_time};

/// Serves `prs` and `issues` newest-update first, `first` per page, like GitHub, and counts calls.
struct Fake {
    prs: Vec<Value>,
    issues: Vec<Value>,
    calls: RefCell<Vec<(String, Option<String>)>>,
}

impl Fake {
    fn new(prs: Vec<Value>, issues: Vec<Value>) -> Fake {
        let by_update = |mut v: Vec<Value>| {
            v.sort_by(|a, b| b["updatedAt"].as_str().cmp(&a["updatedAt"].as_str()));
            v
        };
        Fake { prs: by_update(prs), issues: by_update(issues), calls: RefCell::new(Vec::new()) }
    }
}

impl GraphQl for Fake {
    fn query(&self, query: &str, vars: Value) -> Result<Value, String> {
        let (kind, all) = if query.contains("pullRequests(") { ("pullRequests", &self.prs) } else { ("issues", &self.issues) };
        let after = vars["after"].as_str().map(str::to_string);
        self.calls.borrow_mut().push((kind.to_string(), after.clone()));
        let start: usize = after.map(|a| a.parse().unwrap()).unwrap_or(0);
        let first = vars["first"].as_u64().unwrap() as usize;
        let end = (start + first).min(all.len());
        Ok(json!({ "repository": { kind: {
            "pageInfo": { "hasNextPage": end < all.len(), "endCursor": end.to_string() },
            "nodes": all[start..end],
        }}}))
    }
}

fn pr(number: u64, updated: &str) -> Value {
    json!({
        "number": number, "title": format!("PR {number}"), "state": "MERGED", "isDraft": false,
        "body": "Fixes #1\r\n", "author": { "login": "a" },
        "createdAt": "2026-10-01T00:00:00Z", "updatedAt": updated, "mergedAt": updated, "closedAt": updated,
        "mergeCommit": { "oid": "abc" }, "baseRefName": "main", "headRefName": "fix", "headRefOid": "def",
        "closingIssuesReferences": { "nodes": [{ "number": 1, "repository": { "nameWithOwner": "o/app" } }] },
        "files": { "totalCount": 2, "nodes": [{ "path": "a.rs" }, { "path": "b.rs" }] },
    })
}

fn issue(number: u64, updated: &str) -> Value {
    json!({
        "number": number, "title": format!("Issue {number}"), "state": "OPEN", "stateReason": null,
        "body": "x".repeat(3000), "author": null,
        "createdAt": "2026-10-01T00:00:00Z", "updatedAt": updated, "closedAt": null,
    })
}

fn app() -> App {
    App { name: "app".into(), repo: "o/app".into() }
}

/// 120 PRs an hour apart, so they span three pages of 50.
fn many_prs() -> Vec<Value> {
    (1..=120).map(|n| pr(n, &format_time(parse_time("2026-10-01T00:00:00Z").unwrap() + n as i64 * 3600))).collect()
}

#[test]
fn first_sync_reads_everything_and_parses_records() {
    let tmp = tempfile::tempdir().unwrap();
    let index = Index::new(tmp.path());
    let gh = Fake::new(many_prs(), vec![issue(1, "2026-10-02T00:00:00Z")]);
    let mut state = RepoState::default();
    let c = sync::sync_repo(&gh, &index, &app(), &mut state, false, "2026-10-11T00:00:00Z").unwrap();
    assert_eq!((c.prs_seen, c.prs_changed, c.issues_seen, c.issues_changed), (120, 120, 1, 1));

    let prs = index.prs("app").unwrap();
    assert_eq!(prs.len(), 120);
    let p = &prs[&7];
    assert_eq!(p.closes[0].repo, "o/app");
    assert_eq!(p.paths, vec!["a.rs", "b.rs"]);
    assert_eq!(p.body, "Fixes #1");
    let i = &index.issues("app").unwrap()[&1];
    assert_eq!(i.body.chars().count(), craft_parity::index::BODY_EXCERPT_CHARS + 1, "excerpt plus an ellipsis");
    assert_eq!(state.prs_updated.as_deref(), Some("2026-10-06T00:00:00Z"));
    assert_eq!(state.full_sync, None);
}

#[test]
fn incremental_sync_stops_after_the_cursor_and_only_rewrites_changes() {
    let tmp = tempfile::tempdir().unwrap();
    let index = Index::new(tmp.path());
    let mut state = RepoState::default();
    sync::sync_repo(&Fake::new(many_prs(), vec![]), &index, &app(), &mut state, false, "t").unwrap();
    let before = fs::read(tmp.path().join("index/app/prs.jsonl")).unwrap();

    // Nothing changed: one page per kind, nothing rewritten.
    let gh = Fake::new(many_prs(), vec![]);
    let c = sync::sync_repo(&gh, &index, &app(), &mut state, false, "t").unwrap();
    assert_eq!(c.prs_changed, 0);
    assert_eq!(gh.calls.borrow().iter().filter(|(k, _)| k == "pullRequests").count(), 1);
    assert_eq!(fs::read(tmp.path().join("index/app/prs.jsonl")).unwrap(), before);

    // PR 3 got a new title, so it is now the newest update; one page still covers it.
    let mut prs = many_prs();
    prs[2] = pr(3, "2026-10-07T00:00:00Z");
    prs[2]["title"] = json!("renamed");
    let gh = Fake::new(prs, vec![]);
    let c = sync::sync_repo(&gh, &index, &app(), &mut state, false, "t").unwrap();
    assert_eq!(c.prs_changed, 1);
    assert_eq!(index.prs("app").unwrap()[&3].title, "renamed");
    assert_eq!(state.prs_updated.as_deref(), Some("2026-10-07T00:00:00Z"));
}

#[test]
fn incremental_sync_rereads_the_overlap_window() {
    let tmp = tempfile::tempdir().unwrap();
    let index = Index::new(tmp.path());
    let mut state = RepoState { prs_updated: Some("2026-10-05T00:05:00Z".into()), ..Default::default() };
    // Updated 5 minutes before the cursor: inside the 10 minute overlap, so it must be read.
    let gh = Fake::new(vec![pr(1, "2026-10-05T00:00:00Z")], vec![]);
    let c = sync::sync_repo(&gh, &index, &app(), &mut state, false, "t").unwrap();
    assert_eq!(c.prs_changed, 1);
    assert_eq!(state.prs_updated.as_deref(), Some("2026-10-05T00:05:00Z"), "cursor never moves back");
}

#[test]
fn full_sync_drops_records_github_no_longer_has() {
    let tmp = tempfile::tempdir().unwrap();
    let index = Index::new(tmp.path());
    let mut state = RepoState::default();
    sync::sync_repo(
        &Fake::new(vec![], vec![issue(1, "2026-10-02T00:00:00Z"), issue(2, "2026-10-03T00:00:00Z")]),
        &index,
        &app(),
        &mut state,
        false,
        "t",
    )
    .unwrap();

    // Issue 1 was transferred away. An incremental sync can't tell; a full one can.
    let gh = Fake::new(vec![], vec![issue(2, "2026-10-03T00:00:00Z")]);
    sync::sync_repo(&gh, &index, &app(), &mut state, false, "t").unwrap();
    assert_eq!(index.issues("app").unwrap().len(), 2);
    let c = sync::sync_repo(&gh, &index, &app(), &mut state, true, "2026-10-11T00:00:00Z").unwrap();
    assert_eq!(c.removed, 1);
    assert_eq!(index.issues("app").unwrap().keys().copied().collect::<Vec<_>>(), vec![2]);
    assert_eq!(state.full_sync.as_deref(), Some("2026-10-11T00:00:00Z"));
}

#[test]
fn sync_all_saves_state_per_repo() {
    let tmp = tempfile::tempdir().unwrap();
    let index = Index::new(tmp.path());
    let gh = Fake::new(vec![pr(1, "2026-10-05T00:00:00Z")], vec![]);
    sync::sync_all(&gh, &index, &[app()], false).unwrap();
    assert_eq!(index.state().unwrap()["app"].prs_updated.as_deref(), Some("2026-10-05T00:00:00Z"));
}

#[test]
fn times_round_trip() {
    for t in ["1970-01-01T00:00:00Z", "2000-02-29T23:59:59Z", "2026-10-11T17:14:51Z", "2100-03-01T00:00:00Z"] {
        assert_eq!(format_time(parse_time(t).unwrap()), t);
    }
    assert_eq!(parse_time("2026-10-11T17:14:51Z"), Some(1_791_738_891));
    assert_eq!(parse_time("2026-10-11 17:14:51"), None);
    assert_eq!(parse_time("2026-13-11T17:14:51Z"), None);
}
