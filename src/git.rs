//! Read-only questions to a checkout, asked of one commit through the `git` CLI. The working
//! tree, index and current branch are never looked at, so a checkout on a feature branch with
//! local edits still reports what its upstream has.

use std::path::Path;
use std::process::{Command, Output};

use crate::fix::Check;

fn git(repo: &Path, args: &[&str]) -> Result<Output, String> {
    Command::new("git").arg("-C").arg(repo).args(args).output().map_err(|e| format!("running git: {e}"))
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).trim().to_string()
}

/// The full commit id `rev` names, or `None` if it names nothing.
pub fn resolve(repo: &Path, rev: &str) -> Result<Option<String>, String> {
    let out = git(repo, &["rev-parse", "--verify", "--quiet", &format!("{rev}^{{commit}}")])?;
    Ok(out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string()))
}

/// The first of `revs` that names a commit, with that commit's id.
pub fn resolve_first(repo: &Path, revs: &[&str]) -> Result<Option<(String, String)>, String> {
    for rev in revs {
        if let Some(id) = resolve(repo, rev)? {
            return Ok(Some((rev.to_string(), id)));
        }
    }
    Ok(None)
}

pub fn fetch(repo: &Path) -> Result<(), String> {
    let out = git(repo, &["fetch", "--quiet", "origin"])?;
    if out.status.success() { Ok(()) } else { Err(format!("git fetch: {}", stderr(&out))) }
}

pub fn clone(url: &str, dest: &Path) -> Result<(), String> {
    let out = Command::new("git")
        .args(["clone", "--quiet", "--depth", "1", "--single-branch", url])
        .arg(dest)
        .output()
        .map_err(|e| format!("running git: {e}"))?;
    if out.status.success() { Ok(()) } else { Err(format!("git clone {url}: {}", stderr(&out))) }
}

fn glob(pattern: &str) -> String {
    format!(":(glob){pattern}")
}

/// Whether `check` passes on `commit`.
pub fn passes(repo: &Path, commit: &str, check: &Check) -> Result<bool, String> {
    if let Some(path) = &check.path {
        // ls-tree takes no glob pathspecs, so list the tree and match here.
        let out = git(repo, &["ls-tree", "-r", "--name-only", "-z", commit])?;
        if !out.status.success() {
            return Err(format!("git ls-tree: {}", stderr(&out)));
        }
        return Ok(out.stdout.split(|b| *b == 0).any(|name| glob_match(path.as_bytes(), name)));
    }
    let (pattern, specs) = if let Some(name) = &check.lockfile_crate {
        (format!("^name = \"{}\"$", regex_escape(name)), vec!["Cargo.lock".to_string()])
    } else if let Some(pattern) = &check.grep {
        (pattern.clone(), check.within.iter().map(|p| glob(p)).collect())
    } else {
        return Err("empty check".into());
    };
    let mut args = vec!["grep", "--quiet", "-I", "-E"];
    if check.ignore_case {
        args.push("-i");
    }
    args.extend(["-e", &pattern, commit, "--"]);
    args.extend(specs.iter().map(String::as_str));
    let out = git(repo, &args)?;
    match out.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(format!("git grep: {}", stderr(&out))),
    }
}

/// Matches `name` against a git-style glob: `*` and `?` stay within one folder, `**/` matches any
/// number of folders, `/**` at the end matches everything below.
pub fn glob_match(pattern: &[u8], name: &[u8]) -> bool {
    if let Some(rest) = pattern.strip_prefix(b"**/") {
        return glob_match(rest, name) || name.iter().enumerate().any(|(i, b)| *b == b'/' && glob_match(rest, &name[i + 1..]));
    }
    if pattern == b"/**" {
        return name.len() > 1 && name[0] == b'/';
    }
    match pattern.first() {
        None => name.is_empty(),
        Some(b'*') => {
            let rest = &pattern[1..];
            (0..=name.len()).take_while(|&i| i == 0 || name[i - 1] != b'/').any(|i| glob_match(rest, &name[i..]))
        }
        Some(b'?') => name.first().is_some_and(|c| *c != b'/') && glob_match(&pattern[1..], &name[1..]),
        Some(p) => name.first() == Some(p) && glob_match(&pattern[1..], &name[1..]),
    }
}

fn regex_escape(text: &str) -> String {
    text.chars().fold(String::new(), |mut s, c| {
        if "\\.^$|?*+()[]{}".contains(c) {
            s.push('\\');
        }
        s.push(c);
        s
    })
}

/// Makes sure `commit` and its first parent are in the checkout, fetching them by id if not.
pub fn ensure_commit(repo: &Path, commit: &str) -> Result<String, String> {
    let has = |rev: &str| git(repo, &["cat-file", "-e", &format!("{rev}^{{commit}}")]).map(|o| o.status.success());
    if !has(commit)? || !has(&format!("{commit}^1"))? {
        let out = git(repo, &["fetch", "--quiet", "--depth", "2", "origin", commit])?;
        if !out.status.success() {
            return Err(format!("git fetch {commit}: {}", stderr(&out)));
        }
    }
    resolve(repo, &format!("{commit}^1"))?.ok_or_else(|| format!("{commit} has no parent"))
}

/// The change a commit made against its first parent, without lock files and binary assets.
pub fn diff(repo: &Path, commit: &str) -> Result<String, String> {
    let out = git(
        repo,
        &[
            "diff",
            "--no-color",
            "--no-ext-diff",
            "-U2",
            &format!("{commit}^1"),
            commit,
            "--",
            ".",
            ":(exclude,glob)**/*.lock",
            ":(exclude,glob)**/*.svg",
            ":(exclude,glob)**/*.png",
            ":(exclude,glob)**/*.tiff",
            ":(exclude,glob)**/*.tsv",
            ":(exclude,glob)**/locales/**",
            ":(exclude,glob)**/i18n/**",
        ],
    )?;
    if !out.status.success() {
        return Err(format!("git diff {commit}: {}", stderr(&out)));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// A file's text at `commit`, or `None` when it does not exist there.
pub fn show(repo: &Path, commit: &str, path: &str) -> Result<Option<String>, String> {
    let out = git(repo, &["show", &format!("{commit}:{path}")])?;
    Ok(out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned()))
}

/// Files at `commit` with a line matching `check`'s grep, or the files its path glob matches.
pub fn matching_files(repo: &Path, commit: &str, check: &Check) -> Result<Vec<String>, String> {
    if let Some(path) = &check.path {
        let out = git(repo, &["ls-tree", "-r", "--name-only", "-z", commit])?;
        return Ok(out
            .stdout
            .split(|b| *b == 0)
            .filter(|n| glob_match(path.as_bytes(), n))
            .map(|n| String::from_utf8_lossy(n).into_owned())
            .collect());
    }
    let Some(pattern) = &check.grep else { return Ok(Vec::new()) };
    let mut args = vec!["grep", "-l", "-I", "-E"];
    if check.ignore_case {
        args.push("-i");
    }
    args.extend(["-e", pattern, commit, "--"]);
    let specs: Vec<String> = check.within.iter().map(|p| glob(p)).collect();
    args.extend(specs.iter().map(String::as_str));
    let out = git(repo, &args)?;
    let prefix = format!("{commit}:");
    Ok(String::from_utf8_lossy(&out.stdout).lines().map(|l| l.strip_prefix(&prefix).unwrap_or(l).to_string()).collect())
}
