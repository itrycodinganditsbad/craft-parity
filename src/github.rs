//! GitHub GraphQL: the queries the sync needs and a client that sends them.

use std::process::Command;
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

/// Sends one GraphQL query and returns its `data`. A trait so tests can replay recorded pages.
pub trait GraphQl {
    fn query(&self, query: &str, variables: Value) -> Result<Value, String>;
}

/// Finds a token: `GITHUB_TOKEN`, then `GH_TOKEN`, then `gh auth token`.
pub fn token() -> Result<String, String> {
    for var in ["GITHUB_TOKEN", "GH_TOKEN"] {
        if let Ok(token) = std::env::var(var)
            && !token.trim().is_empty()
        {
            return Ok(token.trim().to_string());
        }
    }
    let out = Command::new("gh").args(["auth", "token"]).output().map_err(|_| "no GITHUB_TOKEN, and `gh` is not installed".to_string())?;
    let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if out.status.success() && !token.is_empty() { Ok(token) } else { Err("no GITHUB_TOKEN, and `gh auth token` gave none".into()) }
}

pub struct Client {
    agent: ureq::Agent,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Client {
        let config = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(60))).http_status_as_error(false).build();
        Client { agent: config.into(), token }
    }
}

/// Attempts per query. GitHub answers 502 to queries that take too long and 403 or 429 when a
/// secondary rate limit trips; both pass after a wait.
const ATTEMPTS: u32 = 5;

impl GraphQl for Client {
    fn query(&self, query: &str, variables: Value) -> Result<Value, String> {
        let body = json!({ "query": query, "variables": variables });
        let mut last = String::new();
        for attempt in 0..ATTEMPTS {
            if attempt > 0 {
                thread::sleep(Duration::from_secs(5 << attempt));
            }
            let response = self
                .agent
                .post("https://api.github.com/graphql")
                .header("Authorization", &format!("Bearer {}", self.token))
                .header("User-Agent", "craft-parity")
                .send_json(&body);
            let mut response = match response {
                Ok(r) => r,
                Err(e) => {
                    last = format!("GitHub GraphQL: {e}");
                    continue;
                }
            };
            let status = response.status().as_u16();
            let text = response.body_mut().read_to_string().map_err(|e| format!("GitHub GraphQL: {e}"))?;
            if matches!(status, 403 | 429 | 500 | 502 | 503 | 504) {
                last = format!("GitHub GraphQL: HTTP {status}: {}", text.chars().take(300).collect::<String>());
                continue;
            }
            if status != 200 {
                return Err(format!("GitHub GraphQL: HTTP {status}: {}", text.chars().take(300).collect::<String>()));
            }
            let value: Value = serde_json::from_str(&text).map_err(|e| format!("GitHub GraphQL: {e}"))?;
            if let Some(errors) = value.get("errors").filter(|e| !e.is_null()) {
                return Err(format!("GitHub GraphQL: {errors}"));
            }
            return Ok(value.get("data").cloned().unwrap_or(Value::Null));
        }
        Err(last)
    }
}

/// Newest first, so an incremental sync stops at the first page older than its cursor.
pub const PRS: &str = r#"
query($owner: String!, $name: String!, $after: String, $first: Int!) {
  repository(owner: $owner, name: $name) {
    pullRequests(first: $first, after: $after, orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo { hasNextPage endCursor }
      nodes {
        number title state isDraft body
        author { login }
        createdAt updatedAt mergedAt closedAt
        mergeCommit { oid }
        baseRefName headRefName headRefOid
        closingIssuesReferences(first: 20) { nodes { number repository { nameWithOwner } } }
        files(first: 100) { totalCount nodes { path } }
      }
    }
  }
  rateLimit { cost remaining resetAt }
}"#;

pub const ISSUES: &str = r#"
query($owner: String!, $name: String!, $after: String, $first: Int!) {
  repository(owner: $owner, name: $name) {
    issues(first: $first, after: $after, orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo { hasNextPage endCursor }
      nodes {
        number title state stateReason body
        author { login }
        createdAt updatedAt closedAt
      }
    }
  }
  rateLimit { cost remaining resetAt }
}"#;
