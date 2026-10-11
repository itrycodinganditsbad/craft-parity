//! Cross-app fix parity for the Craft apps: each fix is a fingerprint in `fixes/*.toml`, checked
//! against every app's default branch.

pub mod classify;
pub mod cluster;
pub mod fix;
pub mod git;
pub mod github;
pub mod html;
pub mod index;
pub mod report;
pub mod scan;
pub mod sync;
