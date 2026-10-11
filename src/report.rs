//! Renders a [`Report`] as Markdown.

use std::fmt::Write;

use crate::scan::{Report, Status};

fn cell(status: &Status) -> &'static str {
    match status {
        Status::Fixed => "✅",
        Status::Missing => "❌",
        Status::NotApplicable | Status::Exempt(_) => "➖",
        Status::Unknown(_) => "❔",
    }
}

/// `storytold/pdfcraft#631` becomes a link to that pull request; anything else is kept as is.
fn reference_link(reference: &str) -> String {
    match reference.split_once('#') {
        Some((repo, num)) if repo.contains('/') && !num.is_empty() && num.bytes().all(|b| b.is_ascii_digit()) => {
            format!("[{reference}](https://github.com/{repo}/pull/{num})")
        }
        _ => reference.to_string(),
    }
}

pub fn markdown(report: &Report) -> String {
    let mut out = String::new();
    out.push_str("# Craft fix parity\n\n");
    out.push_str("Which Craft apps carry a fix that already landed in a sibling app. ");
    out.push_str("✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.\n\n");

    out.push_str("| Fix |");
    for scan in &report.apps {
        let _ = write!(out, " {} |", scan.app.name);
    }
    out.push_str("\n|---|");
    out.push_str(&":-:|".repeat(report.apps.len()));
    out.push('\n');
    for row in &report.rows {
        let _ = write!(out, "| [{}](fixes/{}.toml) |", row.fix.title, row.fix.id);
        for status in &row.statuses {
            let _ = write!(out, " {} |", cell(status));
        }
        out.push('\n');
    }

    out.push_str("\n## Gaps\n");
    let mut any = false;
    for row in &report.rows {
        let missing: Vec<&str> =
            report.apps.iter().zip(&row.statuses).filter(|(_, s)| **s == Status::Missing).map(|(a, _)| a.app.name.as_str()).collect();
        if missing.is_empty() {
            continue;
        }
        any = true;
        let refs: Vec<String> = row.fix.references.iter().map(|r| reference_link(r)).collect();
        let _ =
            write!(out, "\n### {}\n\n{}\n\nMissing in: {}. Port from {}.\n", row.fix.title, row.fix.why.trim(), missing.join(", "), refs.join(", "));
    }
    if !any {
        out.push_str("\nNone.\n");
    }

    let stale: Vec<String> = report.rows.iter().flat_map(|r| r.stale.iter().map(move |a| format!("`{}` in {a}", r.fix.id))).collect();
    if !stale.is_empty() {
        let _ = write!(out, "\n## Stale fingerprints\n\nThese no longer match an app known to carry the fix: {}.\n", stale.join(", "));
    }

    let notes: Vec<String> = report
        .rows
        .iter()
        .flat_map(|r| {
            report.apps.iter().zip(&r.statuses).filter_map(move |(a, s)| match s {
                Status::Exempt(why) => Some(format!("- {} / {}: exempt, {why}", r.fix.id, a.app.name)),
                _ => None,
            })
        })
        .collect();
    if !notes.is_empty() {
        let _ = write!(out, "\n## Exemptions\n\n{}\n", notes.join("\n"));
    }

    out.push_str("\n## Scanned\n\n| App | Ref | Commit |\n|---|---|---|\n");
    for scan in &report.apps {
        match &scan.commit {
            Ok((rev, id)) => {
                let short = id.get(..10).unwrap_or(id);
                let _ = writeln!(
                    out,
                    "| [{}](https://github.com/{}) | {rev} | [{short}](https://github.com/{}/commit/{id}) |",
                    scan.app.name, scan.app.repo, scan.app.repo
                );
            }
            Err(e) => {
                let _ = writeln!(out, "| {} | | not scanned: {e} |", scan.app.name);
            }
        }
    }
    out
}
