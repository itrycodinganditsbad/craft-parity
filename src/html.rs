//! Renders a [`Report`] as one self-contained HTML page: no scripts, no external assets.

use std::fmt::Write;

use crate::cluster::{Cluster, MemberState};
use crate::draft::Draft;
use crate::report::reference_url;
use crate::scan::{Report, Status};

fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Class, visible label and hover text for one cell.
fn cell(status: &Status) -> (&'static str, &'static str, String) {
    match status {
        Status::Fixed => ("fixed", "fixed", "Fixed".into()),
        Status::Missing => ("missing", "missing", "Missing: the fingerprint was not found".into()),
        Status::NotApplicable => ("na", "n/a", "Does not apply: the app lacks the code this fix is about".into()),
        Status::Exempt(why) => ("na", "exempt", format!("Exempt: {why}")),
        Status::Unknown(why) => ("unknown", "?", format!("Not scanned: {why}")),
    }
}

fn reference_html(reference: &str) -> String {
    match reference_url(reference) {
        Some(url) => format!("<a href=\"{}\">{}</a>", esc(&url), esc(reference)),
        None => esc(reference),
    }
}

const STYLE: &str = r#"
:root {
  --bg: #fbfaf8; --surface: #ffffff; --text: #1c1b19; --muted: #6b6862; --line: #e6e3de;
  --fixed: #1f7a4d; --fixed-bg: #e3f3ea; --missing: #b3261e; --missing-bg: #fbe6e4;
  --na: #8a8780; --na-bg: #f1efeb; --unknown: #8a5a00; --unknown-bg: #fbf0d9; --link: #2456b8;
}
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    --bg: #151514; --surface: #1d1d1b; --text: #ecebe8; --muted: #9e9b95; --line: #2f2e2b;
    --fixed: #6fd19c; --fixed-bg: #183526; --missing: #ff8a80; --missing-bg: #3d1d1a;
    --na: #8a8780; --na-bg: #242422; --unknown: #f0c060; --unknown-bg: #3a2f14; --link: #8cb4ff;
    color-scheme: dark;
  }
}
:root[data-theme="dark"] {
    --bg: #151514; --surface: #1d1d1b; --text: #ecebe8; --muted: #9e9b95; --line: #2f2e2b;
    --fixed: #6fd19c; --fixed-bg: #183526; --missing: #ff8a80; --missing-bg: #3d1d1a;
    --na: #8a8780; --na-bg: #242422; --unknown: #f0c060; --unknown-bg: #3a2f14; --link: #8cb4ff;
    color-scheme: dark;
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--text);
  font: 15px/1.5 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; }
main { max-width: 1240px; margin: 0 auto; padding: 40px 16px 64px; }
h1 { font-size: 28px; margin: 0 0 6px; letter-spacing: -0.01em; }
h2 { font-size: 18px; margin: 40px 0 12px; }
p.lede { color: var(--muted); margin: 0 0 24px; max-width: 70ch; }
a { color: var(--link); text-decoration: none; }
a:hover { text-decoration: underline; }
.stats { display: flex; gap: 12px; flex-wrap: wrap; margin-bottom: 28px; }
.stat { background: var(--surface); border: 1px solid var(--line); border-radius: 10px; padding: 12px 16px; min-width: 130px; }
.stat b { display: block; font-size: 24px; font-variant-numeric: tabular-nums; }
.stat span { color: var(--muted); font-size: 13px; }
.stat.alert b { color: var(--missing); }
.scroll { overflow-x: auto; border: 1px solid var(--line); border-radius: 10px; background: var(--surface); }
table { border-collapse: collapse; width: 100%; }
th, td { padding: 8px 6px; border-bottom: 1px solid var(--line); text-align: center; white-space: nowrap; }
tr:last-child td { border-bottom: 0; }
thead th { font-size: 12px; font-weight: 600; color: var(--muted); }
th.fix, td.fix { text-align: left; white-space: normal; min-width: 220px; padding-left: 12px; position: sticky; left: 0; background: var(--surface); }
td.fix a { color: var(--text); font-weight: 500; }
td.fix small { color: var(--muted); display: block; }
.badge { display: inline-block; min-width: 54px; padding: 2px 8px; border-radius: 999px; font-size: 12px; font-weight: 600; }
.badge.fixed { color: var(--fixed); background: var(--fixed-bg); }
.badge.missing { color: var(--missing); background: var(--missing-bg); }
.badge.na { color: var(--na); background: var(--na-bg); font-weight: 500; }
.badge.unknown { color: var(--unknown); background: var(--unknown-bg); }
.gaps { display: grid; gap: 12px; }
.gap { background: var(--surface); border: 1px solid var(--line); border-left: 3px solid var(--missing); border-radius: 10px; padding: 16px 18px; }
.gap h3 { margin: 0 0 6px; font-size: 16px; }
.gap p { margin: 0 0 10px; color: var(--muted); max-width: 80ch; }
.apps { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 10px; }
.apps span { font-size: 13px; padding: 2px 8px; border-radius: 6px; background: var(--missing-bg); color: var(--missing); }
.port { font-size: 13px; }
.stale { border-left-color: var(--unknown); }
.legend { display: flex; gap: 14px; flex-wrap: wrap; margin: 10px 0 0; font-size: 13px; color: var(--muted); }
.scanned td, .scanned th { text-align: left; }
.scanned code { font-size: 12px; }
.group { background: var(--surface); border: 1px solid var(--line); border-radius: 10px; padding: 14px 18px; }
.group .meta { display: flex; gap: 8px; align-items: baseline; flex-wrap: wrap; margin-bottom: 4px; }
.group .area { font-size: 12px; color: var(--muted); border: 1px solid var(--line); border-radius: 6px; padding: 0 6px; }
.group .count { font-size: 12px; color: var(--muted); }
.group h3 { margin: 0 0 8px; font-size: 15px; font-weight: 600; }
.row { display: flex; flex-wrap: wrap; gap: 6px; align-items: baseline; margin: 4px 0; font-size: 13px; }
.row .label { color: var(--muted); min-width: 92px; }
.chip { font-size: 13px; padding: 1px 8px; border-radius: 6px; }
a.chip:hover { text-decoration: none; filter: brightness(0.95); }
.chip.fixed { background: var(--fixed-bg); color: var(--fixed); }
.chip.open { background: var(--unknown-bg); color: var(--unknown); }
.chip.none { background: var(--na-bg); color: var(--na); }
.chip.missing { background: var(--missing-bg); color: var(--missing); }
.chip.unverified { background: var(--missing-bg); color: var(--missing); opacity: 0.7; border: 1px dashed var(--missing); }
.status-line { font-size: 13px; color: var(--muted); margin: 0 0 6px; }
.status-line b { color: var(--fixed); }
.status-line b.no { color: var(--missing); }
pre.toml { background: var(--na-bg); border-radius: 8px; padding: 10px 12px; overflow-x: auto; font-size: 12px; line-height: 1.45; }
.evidence { margin: 4px 0 10px; }
.evidence pre { background: var(--na-bg); border-radius: 6px; padding: 6px 10px; overflow-x: auto; font-size: 12px; white-space: pre-wrap; }
details { margin-top: 6px; font-size: 13px; }
details summary { cursor: pointer; color: var(--muted); }
details li { margin: 3px 0; color: var(--muted); }
details li a { font-variant-numeric: tabular-nums; }
footer { margin-top: 40px; color: var(--muted); font-size: 13px; }
"#;

/// `source` is the base URL fingerprint files link to, e.g. `https://github.com/o/r/blob/main`.
pub fn html(report: &Report, source: &str, groups: &[Cluster], drafts: &[Draft]) -> String {
    let missing_cells = report.rows.iter().flat_map(|r| &r.statuses).filter(|s| **s == Status::Missing).count();
    let fully_ported = report.rows.iter().filter(|r| !r.statuses.contains(&Status::Missing)).count();

    let mut o = String::new();
    o.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n");
    o.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    o.push_str("<title>Craft fix parity</title>\n<style>");
    o.push_str(STYLE);
    o.push_str("</style>\n</head>\n<body>\n<main>\n<h1>Craft fix parity</h1>\n");
    o.push_str("<p class=\"lede\">The Craft apps don't share code, so a fix that lands in one app is often still missing in its siblings. ");
    o.push_str("Each row is a fix with working code somewhere; each <b>missing</b> is a port waiting to happen.</p>\n");

    let _ = writeln!(
        o,
        "<div class=\"stats\"><div class=\"stat\"><b>{}</b><span>fixes tracked</span></div>\
         <div class=\"stat\"><b>{}</b><span>apps scanned</span></div>\
         <div class=\"stat alert\"><b>{missing_cells}</b><span>open gaps</span></div>\
         <div class=\"stat\"><b>{fully_ported}</b><span>fixes everywhere</span></div></div>",
        report.rows.len(),
        report.apps.iter().filter(|a| a.commit.is_ok()).count(),
    );

    o.push_str("<div class=\"scroll\"><table>\n<thead><tr><th class=\"fix\">Fix</th>");
    for scan in &report.apps {
        let _ = write!(o, "<th>{}</th>", esc(&scan.app.name));
    }
    o.push_str("</tr></thead>\n<tbody>\n");
    for row in &report.rows {
        let _ = write!(
            o,
            "<tr><td class=\"fix\"><a href=\"{}/fixes/{}.toml\">{}</a><small>{}</small></td>",
            esc(source),
            esc(&row.fix.id),
            esc(&row.fix.title),
            esc(&row.fix.id)
        );
        for (scan, status) in report.apps.iter().zip(&row.statuses) {
            let (class, label, hover) = cell(status);
            let _ = write!(o, "<td><span class=\"badge {class}\" title=\"{}: {}\">{label}</span></td>", esc(&scan.app.name), esc(&hover));
        }
        o.push_str("</tr>\n");
    }
    o.push_str("</tbody></table></div>\n");
    o.push_str(
        "<div class=\"legend\"><span><span class=\"badge fixed\">fixed</span> fingerprint found</span>\
         <span><span class=\"badge missing\">missing</span> not found</span>\
         <span><span class=\"badge na\">n/a</span> does not apply or exempt</span>\
         <span><span class=\"badge unknown\">?</span> not scanned</span></div>\n",
    );

    let stale: Vec<_> = report.rows.iter().filter(|r| !r.stale.is_empty()).collect();
    if !stale.is_empty() {
        o.push_str("<h2>Stale fingerprints</h2>\n<div class=\"gaps\">\n");
        for row in stale {
            let _ = writeln!(
                o,
                "<div class=\"gap stale\"><h3>{}</h3><p>No longer matches apps known to carry the fix: {}. The fingerprint needs updating.</p></div>",
                esc(&row.fix.title),
                esc(&row.stale.join(", "))
            );
        }
        o.push_str("</div>\n");
    }

    o.push_str("<h2>Gaps</h2>\n<div class=\"gaps\">\n");
    let mut any = false;
    for row in &report.rows {
        let missing: Vec<&str> =
            report.apps.iter().zip(&row.statuses).filter(|(_, s)| **s == Status::Missing).map(|(a, _)| a.app.name.as_str()).collect();
        if missing.is_empty() {
            continue;
        }
        any = true;
        let _ = write!(o, "<div class=\"gap\"><h3>{}</h3><p>{}</p><div class=\"apps\">", esc(&row.fix.title), esc(row.fix.why.trim()));
        for app in missing {
            let _ = write!(o, "<span>{}</span>", esc(app));
        }
        let refs: Vec<String> = row.fix.references.iter().map(|r| reference_html(r)).collect();
        let _ = writeln!(o, "</div><div class=\"port\">Port from {}</div></div>", refs.join(", "));
    }
    if !any {
        o.push_str("<p>None. Every tracked fix is in every app it applies to.</p>\n");
    }
    o.push_str("</div>\n");

    drafts_html(&mut o, report, drafts);
    groups_html(&mut o, report, groups);

    o.push_str("<h2>Scanned</h2>\n<div class=\"scroll\"><table class=\"scanned\">\n<thead><tr><th>App</th><th>Ref</th><th>Commit</th></tr></thead>\n<tbody>\n");
    for scan in &report.apps {
        let repo = esc(&scan.app.repo);
        let name = esc(&scan.app.name);
        match &scan.commit {
            Ok((rev, id)) => {
                let short = id.get(..10).unwrap_or(id);
                let _ = writeln!(
                    o,
                    "<tr><td><a href=\"https://github.com/{repo}\">{name}</a></td><td>{}</td>\
                     <td><a href=\"https://github.com/{repo}/commit/{}\"><code>{}</code></a></td></tr>",
                    esc(rev),
                    esc(id),
                    esc(short)
                );
            }
            Err(e) => {
                let _ = writeln!(o, "<tr><td>{name}</td><td></td><td>not scanned: {}</td></tr>", esc(e));
            }
        }
    }
    o.push_str("</tbody></table></div>\n");
    let _ = write!(
        o,
        "<footer>Built by <a href=\"{0}\">craft-parity</a>. An independent tool, not part of the ArtCraft Team's projects. \
         <a href=\"{0}#add-a-fix\">Add a fix</a>.</footer>\n</main>\n</body>\n</html>\n",
        esc(source.trim_end_matches("/blob/main"))
    );
    o
}

fn member_url(repo: &str, id: &str) -> Option<String> {
    let mut parts = id.rsplitn(3, '/');
    let number = parts.next()?;
    let kind = parts.next()?;
    let path = if kind == "pr" { "pull" } else { "issues" };
    Some(format!("https://github.com/{repo}/{path}/{number}"))
}

/// The same bug fixed or reported in several apps: where it is fixed, where it is only reported,
/// and where nothing is on record.
fn groups_html(o: &mut String, report: &Report, groups: &[Cluster]) {
    let multi: Vec<&Cluster> =
        groups.iter().filter(|g| g.members.iter().map(|m| m.app.as_str()).collect::<std::collections::BTreeSet<_>>().len() >= 2).collect();
    if multi.is_empty() {
        return;
    }
    let repo_of = |app: &str| report.apps.iter().find(|a| a.app.name == app).map(|a| a.app.repo.clone());
    let _ = write!(
        o,
        "<h2>Same bug, several apps</h2>\n<p class=\"lede\">{} groups of fixes and reports that Claude read as the same bug in different apps. \
         An app under <b>reported</b> has an open issue or PR but no merged fix; under <b>no record</b> nothing was found either way, \
         which often means the bug is there and nobody has hit it yet. Groups are leads, not verdicts: a fingerprint has to pass on a \
         real fix before an app counts as fixed above.</p>\n<div class=\"gaps\">\n",
        multi.len()
    );
    for g in multi {
        let first = g.members.iter().find(|m| m.state == MemberState::Fixed).unwrap_or(&g.members[0]);
        let apps_in: std::collections::BTreeSet<&str> = g.members.iter().map(|m| m.app.as_str()).collect();
        let _ = write!(
            o,
            "<div class=\"group\"><div class=\"meta\"><span class=\"area\">{}</span><span class=\"count\">{} apps, {} records</span></div><h3>{}</h3>",
            esc(&g.area.replace('_', " ")),
            apps_in.len(),
            g.members.len(),
            esc(&first.symptom)
        );
        let link_first = |app: &str, state: MemberState| -> String {
            let m = g.members.iter().find(|m| m.app == app && m.state == state).or_else(|| g.members.iter().find(|m| m.app == app));
            match (m, repo_of(app)) {
                (Some(m), Some(repo)) => match member_url(&repo, &m.id) {
                    Some(url) => format!(
                        "<a class=\"chip {}\" href=\"{}\">{}</a>",
                        if state == MemberState::Fixed { "fixed" } else { "open" },
                        esc(&url),
                        esc(app)
                    ),
                    None => esc(app),
                },
                _ => esc(app),
            }
        };
        o.push_str("<div class=\"row\"><span class=\"label\">fixed</span>");
        for app in &g.fixed_in {
            o.push_str(&link_first(app, MemberState::Fixed));
        }
        if g.fixed_in.is_empty() {
            o.push_str("<span class=\"chip none\">none yet</span>");
        }
        o.push_str("</div>");
        if !g.open_in.is_empty() {
            o.push_str("<div class=\"row\"><span class=\"label\">reported</span>");
            for app in &g.open_in {
                o.push_str(&link_first(app, MemberState::OpenIssue));
            }
            o.push_str("</div>");
        }
        let none: Vec<&str> = report.apps.iter().map(|a| a.app.name.as_str()).filter(|a| !apps_in.contains(a)).collect();
        if !none.is_empty() {
            o.push_str("<div class=\"row\"><span class=\"label\">no record</span>");
            for app in none {
                let _ = write!(o, "<span class=\"chip none\">{}</span>", esc(app));
            }
            o.push_str("</div>");
        }
        let _ = write!(o, "<details><summary>All {} records</summary><ul>", g.members.len());
        for m in &g.members {
            let label = esc(&m.id);
            let link = repo_of(&m.app).and_then(|r| member_url(&r, &m.id)).map(|u| format!("<a href=\"{}\">{label}</a>", esc(&u))).unwrap_or(label);
            let _ = write!(o, "<li>{link}: {}</li>", esc(&m.symptom));
        }
        o.push_str("</ul></details></div>\n");
    }
    o.push_str("</div>\n");
}

/// Drafted fingerprints: whether they proved out, where each app stands, and what Claude found in
/// the code of every app that reads missing.
fn drafts_html(o: &mut String, report: &Report, drafts: &[Draft]) {
    if drafts.is_empty() {
        return;
    }
    let repo_of = |app: &str| report.apps.iter().find(|a| a.app.name == app).map(|a| a.app.repo.clone());
    let proven = drafts.iter().filter(|d| d.proven).count();
    let _ = write!(
        o,
        "<h2>Proposed fingerprints</h2>\n<p class=\"lede\">{} drafted by Claude from the fix diffs, {proven} proven: on every fix PR in the group \
         the fingerprint reads missing before the merge and fixed after it. For each app that still reads missing, Claude then read that \
         app's code: <b>bug present</b> gaps quote the lines; a dashed chip was not confirmed. A fingerprint counts in the matrix above \
         only once it is added to <code>fixes/</code>.</p>\n<div class=\"gaps\">\n",
        drafts.len()
    );
    let mut sorted: Vec<&Draft> = drafts.iter().collect();
    sorted.sort_by(|a, b| {
        let gaps = |d: &Draft| d.verified.values().filter(|v| v.verdict == "present").count();
        b.proven.cmp(&a.proven).then_with(|| gaps(b).cmp(&gaps(a))).then_with(|| a.key.cmp(&b.key))
    });
    for d in sorted {
        let title = d
            .toml
            .lines()
            .find_map(|l| l.strip_prefix("title = "))
            .map(|t| t.trim_matches('"').replace("\\\"", "\""))
            .unwrap_or_else(|| d.key.clone());
        let why = d.toml.lines().find_map(|l| l.strip_prefix("why = ")).map(|t| t.trim_matches('"').replace("\\\"", "\"")).unwrap_or_default();
        let _ = write!(
            o,
            "<div class=\"group\"><div class=\"meta\"><span class=\"area\">{}</span><span class=\"count\">{}</span></div><h3>{}</h3>",
            esc(&d.area.replace('_', " ")),
            esc(&d.key),
            esc(&title)
        );
        if !why.is_empty() {
            let _ = write!(o, "<p class=\"status-line\">{}</p>", esc(&why));
        }
        let proofs: Vec<String> = d
            .proof
            .iter()
            .map(|p| match repo_of(&p.app) {
                Some(r) => {
                    format!("<a href=\"https://github.com/{}/pull/{}\">{} #{}</a>{}", esc(&r), p.pr, esc(&p.app), p.pr, if p.ok { "" } else { " ✗" })
                }
                None => esc(&p.app),
            })
            .collect();
        let _ = write!(
            o,
            "<p class=\"status-line\">{} after {} attempt(s) on {}</p>",
            if d.proven { "<b>Proven</b>".to_string() } else { "<b class=\"no\">Not proven</b>".to_string() },
            d.attempts,
            proofs.join(", ")
        );
        if d.proven {
            let mut fixed = Vec::new();
            let mut present = Vec::new();
            let mut unverified = Vec::new();
            let mut other = Vec::new();
            for (app, st) in &d.matrix {
                match st {
                    Status::Fixed => fixed.push(app.as_str()),
                    Status::Missing => match d.verified.get(app).map(|v| v.verdict.as_str()) {
                        Some("present") => present.push(app.as_str()),
                        _ => unverified.push(app.as_str()),
                    },
                    _ => other.push(app.as_str()),
                }
            }
            let row = |o: &mut String, label: &str, class: &str, apps: &[&str]| {
                if apps.is_empty() {
                    return;
                }
                let _ = write!(o, "<div class=\"row\"><span class=\"label\">{label}</span>");
                for a in apps {
                    let _ = write!(o, "<span class=\"chip {class}\">{}</span>", esc(a));
                }
                o.push_str("</div>");
            };
            row(o, "bug present", "missing", &present);
            row(o, "unconfirmed", "unverified", &unverified);
            row(o, "fixed", "fixed", &fixed);
            row(o, "n/a", "none", &other);
            if !d.verified.is_empty() {
                let _ = write!(o, "<details><summary>What Claude found in {} app(s)</summary>", d.verified.len());
                for (app, v) in &d.verified {
                    let _ = write!(
                        o,
                        "<div class=\"evidence\"><b>{}</b>: {}. {}{}<pre>{}</pre></div>",
                        esc(app),
                        esc(&v.verdict.replace('_', " ")),
                        esc(&v.explanation),
                        if v.alternative.is_some() { " An alternative check for this was added." } else { "" },
                        esc(&v.evidence)
                    );
                }
                o.push_str("</details>");
            }
        }
        let _ = writeln!(o, "<details><summary>Fingerprint TOML</summary><pre class=\"toml\">{}</pre></details></div>", esc(&d.toml));
    }
    o.push_str("</div>\n");
}
