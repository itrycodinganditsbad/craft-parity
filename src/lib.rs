//! Cross-app fix parity for the Craft apps: each fix is a fingerprint in `fixes/*.toml`, checked
//! against every app's default branch.

pub mod fix;
pub mod git;
pub mod report;
pub mod scan;
