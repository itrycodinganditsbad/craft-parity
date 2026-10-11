use craft_parity::git::glob_match;

fn m(pattern: &str, name: &str) -> bool {
    glob_match(pattern.as_bytes(), name.as_bytes())
}

#[test]
fn star_stays_in_one_folder() {
    assert!(m("packaging/macos/dmg/background.*", "packaging/macos/dmg/background.tiff"));
    assert!(m("apps/*/src/logging.rs", "apps/photocraft/src/logging.rs"));
    assert!(!m("apps/*/src/logging.rs", "apps/a/b/src/logging.rs"));
    assert!(!m("*.rs", "src/main.rs"));
}

#[test]
fn double_star_crosses_folders() {
    assert!(m("**/*.rs", "main.rs"));
    assert!(m("**/*.rs", "crates/a/src/lib.rs"));
    assert!(!m("**/*.rs", "crates/a/src/lib.rsx"));
    assert!(m("crates/**", "crates/a/b.rs"));
    assert!(!m("crates/**", "crates"));
    assert!(m("crates/**/lib.rs", "crates/lib.rs"));
}

#[test]
fn question_mark_is_one_character() {
    assert!(m("a?c", "abc"));
    assert!(!m("a?c", "a/c"));
    assert!(!m("a?c", "ac"));
}
