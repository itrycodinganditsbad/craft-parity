//! The classify loop against a fake model.

use std::collections::BTreeMap;
use std::sync::Mutex;

use serde_json::json;

use craft_parity::classify::{self, Classifier, Item, Labels, Options, Store, Usage};
use craft_parity::fix::App;
use craft_parity::index::{Index, Issue, IssueRef, PullRequest};

fn labels(symptom: &str) -> Labels {
    Labels {
        kind: "bug_fix".into(),
        scope: "suite_wide".into(),
        area: "gpu_rendering".into(),
        platforms: vec!["windows".into()],
        symptom: symptom.into(),
        root_cause: String::new(),
        fixed_markers: vec![],
        bug_markers: vec![],
        confidence: 0.9,
    }
}

/// Labels every item except those listed in `skip`, and fails on call number `fail_on`.
struct Fake {
    skip: Vec<String>,
    fail_on: Option<usize>,
    seen: Mutex<Vec<Vec<String>>>,
}

impl Fake {
    fn new() -> Fake {
        Fake { skip: vec![], fail_on: None, seen: Mutex::new(vec![]) }
    }
}

impl Classifier for Fake {
    fn model(&self) -> &str {
        "fake-model"
    }
    fn classify(&self, batch: &[Item]) -> Result<(BTreeMap<String, Labels>, Usage), String> {
        let mut seen = self.seen.lock().unwrap();
        seen.push(batch.iter().map(|i| i.id.clone()).collect());
        if self.fail_on == Some(seen.len()) {
            return Err("You've hit your session limit · resets 3:45pm".into());
        }
        let out = batch.iter().filter(|i| !self.skip.contains(&i.id)).map(|i| (i.id.clone(), labels(&i.id))).collect();
        Ok((out, Usage { calls: 1, input_tokens: 100, output_tokens: 10, cost_usd: 0.01 }))
    }
}

fn pr(number: u64, state: &str, draft: bool, closes: &[u64]) -> PullRequest {
    PullRequest {
        number,
        title: format!("PR {number}"),
        state: state.into(),
        draft,
        author: None,
        created_at: "2026-10-01T00:00:00Z".into(),
        updated_at: format!("2026-10-0{}T00:00:00Z", number % 9 + 1),
        merged_at: (state == "MERGED").then(|| format!("2026-10-0{}T00:00:00Z", number % 9 + 1)),
        closed_at: None,
        merge_commit: None,
        base_ref: "main".into(),
        head_ref: "x".into(),
        head_commit: None,
        closes: closes.iter().map(|n| IssueRef { repo: "o/app".into(), number: *n }).collect(),
        paths: vec!["src/main.rs".into()],
        paths_total: 1,
        body: "body".into(),
    }
}

fn issue(number: u64, state: &str) -> Issue {
    Issue {
        number,
        title: format!("Issue {number}"),
        state: state.into(),
        state_reason: None,
        author: None,
        created_at: "2026-10-01T00:00:00Z".into(),
        updated_at: "2026-10-01T00:00:00Z".into(),
        closed_at: None,
        body: "it breaks".into(),
    }
}

fn app() -> App {
    App { name: "app".into(), repo: "o/app".into() }
}

fn setup(dir: &std::path::Path) -> Index {
    let index = Index::new(dir);
    let prs: BTreeMap<u64, PullRequest> = [
        pr(1, "MERGED", false, &[10]),
        pr(2, "MERGED", false, &[]),
        pr(3, "OPEN", false, &[]),
        pr(4, "OPEN", true, &[]),    // draft: skipped
        pr(5, "CLOSED", false, &[]), // closed unmerged: skipped
    ]
    .into_iter()
    .map(|p| (p.number, p))
    .collect();
    index.write_prs("app", &prs).unwrap();
    // 10 is closed by merged PR 1, so it is covered by that record; 11 is open and unfixed; 12 closed.
    let issues: BTreeMap<u64, Issue> = [issue(10, "OPEN"), issue(11, "OPEN"), issue(12, "CLOSED")].into_iter().map(|i| (i.number, i)).collect();
    index.write_issues("app", &issues).unwrap();
    index
}

#[test]
fn picks_the_right_records_and_folds_closed_issues_into_their_fix() {
    let tmp = tempfile::tempdir().unwrap();
    let index = setup(tmp.path());
    let items = classify::items(&index, &app(), "m").unwrap();
    let ids: Vec<&str> = items.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(ids, vec!["app/pr/1", "app/pr/2", "app/pr/3", "app/issue/11"]);
    assert!(items[0].text.contains("Closes issue #10: Issue 10"), "{}", items[0].text);
    assert!(items[0].text.contains("(merged)"));
}

#[test]
fn resumes_and_only_resends_what_changed() {
    let tmp = tempfile::tempdir().unwrap();
    let index = setup(tmp.path());
    let store = Store::new(tmp.path());
    let opts = Options { batch_size: 2, jobs: 2, max_calls: None };

    let fake = Fake::new();
    let out = classify::run(&fake, &index, &store, &[app()], &opts).unwrap();
    assert_eq!((out.pending, out.classified, out.calls), (4, 4, 2));
    assert_eq!(store.load("app").unwrap().len(), 4);

    // Nothing changed: nothing sent.
    let fake = Fake::new();
    let out = classify::run(&fake, &index, &store, &[app()], &opts).unwrap();
    assert_eq!((out.pending, out.calls), (0, 0));
    assert!(fake.seen.lock().unwrap().is_empty());

    // PR 2's title changed: only it goes again.
    let mut prs = index.prs("app").unwrap();
    prs.get_mut(&2).unwrap().title = "new title".into();
    index.write_prs("app", &prs).unwrap();
    let fake = Fake::new();
    let out = classify::run(&fake, &index, &store, &[app()], &opts).unwrap();
    assert_eq!(out.pending, 1);
    assert_eq!(*fake.seen.lock().unwrap(), vec![vec!["app/pr/2".to_string()]]);
}

#[test]
fn a_failed_call_stops_the_run_and_keeps_finished_batches() {
    let tmp = tempfile::tempdir().unwrap();
    let index = setup(tmp.path());
    let store = Store::new(tmp.path());
    let opts = Options { batch_size: 1, jobs: 1, max_calls: None };

    let fake = Fake { fail_on: Some(3), ..Fake::new() };
    let out = classify::run(&fake, &index, &store, &[app()], &opts).unwrap();
    assert_eq!(out.classified, 2);
    assert!(out.stopped.as_deref().unwrap().contains("session limit"));
    assert_eq!(fake.seen.lock().unwrap().len(), 3, "no calls after the failure");

    // The next run picks up the other two, merged fixes first already done.
    let fake = Fake::new();
    let out = classify::run(&fake, &index, &store, &[app()], &opts).unwrap();
    assert_eq!(out.classified, 2);
    assert_eq!(store.load("app").unwrap().len(), 4);
}

#[test]
fn records_the_model_leaves_out_are_retried() {
    let tmp = tempfile::tempdir().unwrap();
    let index = setup(tmp.path());
    let store = Store::new(tmp.path());
    let opts = Options { batch_size: 10, jobs: 1, max_calls: None };
    let fake = Fake { skip: vec!["app/pr/3".into()], ..Fake::new() };
    classify::run(&fake, &index, &store, &[app()], &opts).unwrap();
    let out = classify::run(&Fake::new(), &index, &store, &[app()], &opts).unwrap();
    assert_eq!(out.pending, 1);
}

#[test]
fn parse_results_drops_unknown_ids_and_bad_enums() {
    let tmp = tempfile::tempdir().unwrap();
    let index = setup(tmp.path());
    let items = classify::items(&index, &app(), "m").unwrap();
    let good = json!({"id": "app/pr/1", "kind": "bug_fix", "scope": "suite_wide", "area": "gpu_rendering", "platforms": ["windows"],
        "symptom": "s", "root_cause": "r", "fixed_markers": [], "bug_markers": [], "confidence": 0.5});
    let mut stranger = good.clone();
    stranger["id"] = json!("other/pr/9");
    let mut bad = good.clone();
    bad["id"] = json!("app/pr/2");
    bad["scope"] = json!("galaxy_wide");
    let parsed = classify::parse_results(&json!({ "results": [good, stranger, bad] }), &items);
    assert_eq!(parsed.keys().collect::<Vec<_>>(), vec!["app/pr/1"]);
}

#[test]
fn schema_lists_every_required_field() {
    let s = classify::schema();
    let item = &s["properties"]["results"]["items"];
    let props: Vec<&String> = item["properties"].as_object().unwrap().keys().collect();
    assert_eq!(item["required"].as_array().unwrap().len(), props.len());
}
