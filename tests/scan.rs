//! End to end: fingerprints against throwaway git repos.

use std::fs;
use std::path::Path;
use std::process::Command;

use craft_parity::fix::{self, App};
use craft_parity::scan::{self, DEFAULT_REVS, Status};

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(repo).args(args).output().unwrap();
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A repo with `files` committed on `main`.
fn repo(root: &Path, name: &str, files: &[(&str, &str)]) {
    let dir = root.join(name);
    fs::create_dir_all(&dir).unwrap();
    git(&dir, &["init", "-q", "-b", "main"]);
    for (path, text) in files {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    git(&dir, &["add", "-A"]);
    git(&dir, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "--allow-empty", "-m", "init"]);
}

fn app(name: &str) -> App {
    App { name: name.into(), repo: format!("example/{name}") }
}

const GPU: &str = r#"
title = "GPU"
why = "w"
references = ["example/fixed#1"]
known_fixed = ["fixed"]
exempt = { exempt = "no window" }

[[applies_if]]
lockfile_crate = "wgpu"

[[fixed_if]]
grep = "enumdisplaydevices"
ignore_case = true
in = ["**/*.rs"]

[[fixed_if]]
path = "apps/*/src/gpu.rs"
"#;

const WGPU_LOCK: &str = "[[package]]\nname = \"wgpu\"\nversion = \"1\"\n";

#[test]
fn statuses_for_each_kind_of_app() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    repo(root, "fixed", &[("Cargo.lock", WGPU_LOCK), ("apps/x/src/gpu.rs", "// EnumDisplayDevices\n")]);
    repo(root, "missing", &[("Cargo.lock", WGPU_LOCK), ("apps/x/src/gpu.rs", "// nothing\n")]);
    // Matching text in the wrong place does not count: the grep is limited to *.rs.
    repo(root, "half", &[("Cargo.lock", WGPU_LOCK), ("notes.md", "EnumDisplayDevices\n"), ("apps/x/src/gpu.rs", "")]);
    repo(root, "nogpu", &[("Cargo.lock", "[[package]]\nname = \"wgpu-types\"\n"), ("src/main.rs", "EnumDisplayDevices")]);
    repo(root, "exempt", &[]);

    let fixes_dir = root.join("fixes");
    fs::create_dir(&fixes_dir).unwrap();
    fs::write(fixes_dir.join("gpu.toml"), GPU).unwrap();
    let apps: Vec<App> = ["fixed", "missing", "half", "nogpu", "exempt", "absent"].map(app).to_vec();
    let fixes = fix::load_fixes(&fixes_dir, &apps).unwrap();

    let located: Vec<_> = apps.iter().map(|a| scan::locate(root, a, DEFAULT_REVS)).collect();
    let report = scan::scan(root, &located, &fixes);
    let row = &report.rows[0];
    assert_eq!(row.fix.id, "gpu");
    assert_eq!(&row.statuses[..5], &[Status::Fixed, Status::Missing, Status::Missing, Status::NotApplicable, Status::Exempt("no window".into()),]);
    assert!(matches!(row.statuses[5], Status::Unknown(_)));
    assert!(row.stale.is_empty());

    let md = craft_parity::report::markdown(&report);
    assert!(md.contains("Missing in: missing, half."), "{md}");
    assert!(md.contains("[example/fixed#1](https://github.com/example/fixed/pull/1)"), "{md}");
}

#[test]
fn reads_the_upstream_branch_not_the_checked_out_one() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    repo(root, "upstream", &[("Cargo.lock", WGPU_LOCK), ("apps/x/src/gpu.rs", "EnumDisplayDevices")]);
    let up = root.join("upstream");
    let out = Command::new("git").arg("clone").arg("-q").arg(&up).arg(root.join("fixed")).output().unwrap();
    assert!(out.status.success());
    // A local branch that drops the fix must not hide it.
    let local = root.join("fixed");
    git(&local, &["checkout", "-q", "-b", "wip"]);
    fs::write(local.join("apps/x/src/gpu.rs"), "gone").unwrap();
    git(&local, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qam", "wip"]);

    let fixes_dir = root.join("fixes");
    fs::create_dir(&fixes_dir).unwrap();
    fs::write(fixes_dir.join("gpu.toml"), GPU.replace("exempt = { exempt = \"no window\" }", "")).unwrap();
    let apps = vec![app("fixed")];
    let fixes = fix::load_fixes(&fixes_dir, &apps).unwrap();
    let located: Vec<_> = apps.iter().map(|a| scan::locate(root, a, DEFAULT_REVS)).collect();
    assert_eq!(located[0].commit.as_ref().unwrap().0, "origin/HEAD");
    let report = scan::scan(root, &located, &fixes);
    assert_eq!(report.rows[0].statuses, vec![Status::Fixed]);
    assert!(!report.is_stale());

    // Reading the local branch instead shows the fix gone, and flags the fingerprint as stale
    // because `fixed` is in known_fixed.
    let located: Vec<_> = apps.iter().map(|a| scan::locate(root, a, &["HEAD"])).collect();
    let report = scan::scan(root, &located, &fixes);
    assert_eq!(report.rows[0].statuses, vec![Status::Missing]);
    assert_eq!(report.rows[0].stale, vec!["fixed".to_string()]);
}

#[test]
fn rejects_bad_fingerprints() {
    let tmp = tempfile::tempdir().unwrap();
    let apps = vec![app("a")];
    let cases = [
        ("both", "title='t'\nwhy='w'\nreferences=[]\nknown_fixed=['a']\n[[fixed_if]]\npath='x'\ngrep='y'\n", "exactly one"),
        ("unknown_app", "title='t'\nwhy='w'\nreferences=[]\nknown_fixed=['zzz']\n[[fixed_if]]\npath='x'\n", "unknown app"),
        ("no_known", "title='t'\nwhy='w'\nreferences=[]\nknown_fixed=[]\n[[fixed_if]]\npath='x'\n", "known_fixed"),
        ("typo", "title='t'\nwhy='w'\nreferences=[]\nknown_fixed=['a']\n[[fixed_if]]\npaht='x'\n", "unknown field"),
        ("in_on_path", "title='t'\nwhy='w'\nreferences=[]\nknown_fixed=['a']\n[[fixed_if]]\npath='x'\nin=['y']\n", "only go with"),
    ];
    for (name, text, want) in cases {
        let dir = tmp.path().join(name);
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("f.toml"), text).unwrap();
        let err = fix::load_fixes(&dir, &apps).unwrap_err();
        assert!(err.contains(want), "{name}: {err}");
    }
}

#[test]
fn shipped_fingerprints_load() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let apps = fix::load_apps(&here.join("apps.toml")).unwrap();
    let fixes = fix::load_fixes(&here.join("fixes"), &apps).unwrap();
    assert!(!fixes.is_empty());
}
