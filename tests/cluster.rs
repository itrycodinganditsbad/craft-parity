//! Clustering of classified records across apps.

use craft_parity::classify::{Classified, Labels};
use craft_parity::cluster::{self, MemberState};

#[allow(clippy::too_many_arguments)]
fn rec(app: &str, n: u64, kind: &str, scope: &str, area: &str, symptom: &str, conf: f64, state: MemberState) -> (String, Classified, MemberState) {
    let labels = Labels {
        kind: kind.into(),
        scope: scope.into(),
        area: area.into(),
        platforms: vec!["windows".into()],
        symptom: symptom.into(),
        root_cause: String::new(),
        fixed_markers: vec![],
        bug_markers: vec![],
        confidence: conf,
    };
    (app.into(), Classified { id: format!("{app}/pr/{n}"), input: "h".into(), model: "m".into(), labels }, state)
}

fn corpus() -> Vec<(String, Classified, MemberState)> {
    use MemberState::*;
    vec![
        rec(
            "pdfcraft",
            631,
            "bug_fix",
            "suite_wide",
            "gpu_rendering",
            "On Windows desktops with two GPUs every monitor goes black when the window opens on the integrated GPU.",
            0.9,
            Fixed,
        ),
        rec(
            "vectorcraft",
            851,
            "bug_fix",
            "suite_wide",
            "gpu_rendering",
            "On Windows desktops with two GPUs the window opens on the integrated GPU and the monitors go black.",
            0.9,
            Fixed,
        ),
        rec(
            "filmcraft",
            9,
            "bug_fix",
            "unclear",
            "gpu_rendering",
            "Monitors go black on a Windows desktop with two GPUs when the integrated GPU is picked.",
            0.5,
            OpenIssue,
        ),
        // Same words, different area: must not join.
        rec("photocraft", 1, "bug_fix", "suite_wide", "packaging_install", "On Windows desktops with two GPUs the installer goes black.", 0.9, Fixed),
        // App-specific and features never take part.
        rec("lightcraft", 2, "bug_fix", "app_specific", "gpu_rendering", "On Windows desktops with two GPUs every monitor goes black.", 0.9, Fixed),
        rec("cadcraft", 3, "feature", "suite_wide", "gpu_rendering", "On Windows desktops with two GPUs every monitor goes black.", 0.9, Fixed),
        // Unrelated suite-wide fix, alone, confident: a candidate by itself.
        rec(
            "wordcraft",
            4,
            "bug_fix",
            "suite_wide",
            "logging_diagnostics",
            "Started from the Start menu the log records are lost because there is no terminal.",
            0.8,
            Fixed,
        ),
        // Unrelated, alone, unsure: not a candidate.
        rec("gridcraft", 5, "bug_fix", "unclear", "ui_theme", "Tooltips flicker over the formula bar.", 0.4, Fixed),
    ]
}

#[test]
fn tokens_drop_noise_and_plurals() {
    assert_eq!(cluster::tokens("The GPUs drive the monitors, it's black"), vec!["gpu", "drive", "monitor", "black"]);
}

#[test]
fn same_bug_in_three_apps_forms_one_cluster() {
    let all = cluster::clusters(&corpus(), cluster::DEFAULT_THRESHOLD);
    let gpu = &all[0];
    let ids: Vec<&str> = gpu.members.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(ids, vec!["pdfcraft/pr/631", "vectorcraft/pr/851", "filmcraft/pr/9"], "fixed first, then open");
    assert_eq!(gpu.fixed_in, vec!["pdfcraft", "vectorcraft"]);
    assert_eq!(gpu.open_in, vec!["filmcraft"]);
    assert!(gpu.is_candidate());
    assert!(all.iter().all(|c| c.members.iter().all(|m| !m.id.starts_with("lightcraft") && !m.id.starts_with("cadcraft"))));
    assert!(all.iter().any(|c| c.members.len() == 1 && c.members[0].id == "photocraft/pr/1"), "other area stays apart");
}

#[test]
fn candidates_need_a_fix_and_either_two_apps_or_confidence() {
    let all = cluster::clusters(&corpus(), cluster::DEFAULT_THRESHOLD);
    let candidate = |id: &str| all.iter().find(|c| c.members.iter().any(|m| m.id == id)).unwrap().is_candidate();
    assert!(candidate("wordcraft/pr/4"));
    assert!(!candidate("gridcraft/pr/5"));
}

#[test]
fn output_is_stable() {
    let a = serde_json::to_string(&cluster::clusters(&corpus(), 0.42)).unwrap();
    let mut shuffled = corpus();
    shuffled.reverse();
    let b = serde_json::to_string(&cluster::clusters(&shuffled, 0.42)).unwrap();
    assert_eq!(a, b);
}
