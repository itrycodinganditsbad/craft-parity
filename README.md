# craft-parity

The Craft apps (PhotoCraft, LightCraft, FilmCraft, PdfCraft and the rest) don't share code. Each
repo fixes its own bugs, so a fix that lands in one app is often still missing in its siblings.
PdfCraft and VectorCraft pick the GPU that drives the primary display on Windows; ten other apps
can still pick one that drives no monitor at all.

craft-parity keeps a fingerprint for each such fix and checks every app's default branch for it.
The result is [PARITY.md](PARITY.md), rebuilt daily by CI. Every ❌ is a fix with working code to
port from.

This is an independent tool, not part of the ArtCraft Team's projects.

## Run it

With the apps checked out next to this repo (`../photocraft`, `../pdfcraft`, ...):

```sh
cargo run -- --fetch            # fetch each checkout, print the report
cargo run -- --out PARITY.md
cargo run -- --only windows-primary-gpu --format json
```

`--root DIR` points somewhere else, and `--clone` shallow-clones any app that's missing there. Each
checkout is read at `origin/HEAD` (falling back to `origin/main`, then `HEAD`) through `git
ls-tree` and `git grep`. Your current branch, local edits and index are ignored, so a checkout on a
feature branch still reports upstream. `--rev` picks another ref.

Exit codes: 0 for a report, 1 for a broken setup, 2 when a fingerprint went stale (see below).

## Add a fix

One TOML file in `fixes/`; the file name is the id.

```toml
title = "Windows: draw on the GPU that drives the primary display"
why = "What goes wrong without it, in a sentence or two."
references = ["storytold/pdfcraft#631"]    # the PRs to port from
known_fixed = ["pdfcraft", "vectorcraft"]  # apps you checked by hand

[[applies_if]]                 # all must pass, or the app shows ➖
lockfile_crate = "wgpu"

[[fixed_if]]                   # all must pass for ✅
grep = "EnumDisplayDevices"    # extended regex
in = ["**/*.rs"]               # optional globs; ignore_case = true also works

# [[fixed_if]]
# path = "packaging/macos/dmg/background.*"

# [exempt]
# soundcraft = "draws no GPU surface"
```

A check is exactly one of `path` (a file matching the glob exists), `grep` (a line matches) or
`lockfile_crate` (the root `Cargo.lock` has that package). In globs, `*` stays inside one folder
and `**` crosses folders.

`known_fixed` keeps fingerprints honest. If an app listed there stops matching (the code was
renamed or moved), the run exits 2 and PARITY.md lists the fingerprint under "Stale fingerprints".

Prefer something that only exists because of the fix, like a Windows API call, a file or a
function name, over words that happen to show up in the fix's commit message. Check the result
against the apps you know have the fix and one you know doesn't before you open a PR.

## License

MIT OR Apache-2.0.
