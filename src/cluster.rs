//! Groups classified records that describe the same bug in different apps. Deterministic: TF-IDF
//! cosine over the app-free `symptom` and `root_cause` sentences, within one area, joined by
//! union-find. No model call.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::classify::Classified;

/// Two records join a cluster when their similarity reaches this.
pub const DEFAULT_THRESHOLD: f64 = 0.35;

const STOPWORDS: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "but", "by", "can", "does", "doesn", "don", "for", "from", "has", "have", "if", "in", "into", "is",
    "it", "its", "of", "on", "or", "so", "such", "than", "that", "the", "their", "then", "there", "these", "they", "this", "to", "too", "was",
    "were", "when", "which", "while", "with", "without", "app", "apps", "user", "users", "instead", "still", "only", "not", "no", "t", "s", "one",
    "some", "any", "all", "same",
];

/// Lowercase words of 2+ letters minus stop words, with a light plural strip.
pub fn tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .map(str::to_lowercase)
        .filter(|w| w.len() > 1 && !STOPWORDS.contains(&w.as_str()))
        .map(|w| if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") { w[..w.len() - 1].to_string() } else { w })
        .collect()
}

struct Vector(BTreeMap<String, f64>);

impl Vector {
    fn cosine(&self, other: &Vector) -> f64 {
        let (small, big) = if self.0.len() <= other.0.len() { (&self.0, &other.0) } else { (&other.0, &self.0) };
        small.iter().filter_map(|(k, v)| big.get(k).map(|w| v * w)).sum()
    }
}

/// Unit TF-IDF vectors for each text.
fn vectors(texts: &[Vec<String>]) -> Vec<Vector> {
    let mut df: BTreeMap<&str, usize> = BTreeMap::new();
    for t in texts {
        for w in t.iter().collect::<BTreeSet<_>>() {
            *df.entry(w.as_str()).or_default() += 1;
        }
    }
    let n = texts.len() as f64;
    texts
        .iter()
        .map(|t| {
            let mut v: BTreeMap<String, f64> = BTreeMap::new();
            for w in t {
                *v.entry(w.clone()).or_default() += 1.0;
            }
            for (w, x) in v.iter_mut() {
                let idf = (1.0 + n / (1.0 + df[w.as_str()] as f64)).ln();
                *x *= idf;
            }
            let norm = v.values().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                v.values_mut().for_each(|x| *x /= norm);
            }
            Vector(v)
        })
        .collect()
}

fn find(parent: &mut [usize], i: usize) -> usize {
    let mut r = i;
    while parent[r] != r {
        r = parent[r];
    }
    let mut i = i;
    while parent[i] != r {
        let next = parent[i];
        parent[i] = r;
        i = next;
    }
    r
}

#[derive(Debug, Clone, Serialize)]
pub struct Member {
    pub app: String,
    pub id: String,
    pub symptom: String,
    pub confidence: f64,
    /// Merged fix, open PR or open issue.
    pub state: MemberState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberState {
    Fixed,
    OpenPr,
    OpenIssue,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cluster {
    pub area: String,
    pub members: Vec<Member>,
    /// Apps with a merged fix in this cluster.
    pub fixed_in: Vec<String>,
    /// Apps with only an open PR or issue in it.
    pub open_in: Vec<String>,
}

impl Cluster {
    /// Worth a fingerprint: the same bug seen in two or more apps, or one confident suite-wide fix.
    pub fn is_candidate(&self) -> bool {
        let apps: BTreeSet<&str> = self.members.iter().map(|m| m.app.as_str()).collect();
        !self.fixed_in.is_empty() && (apps.len() >= 2 || self.members.iter().any(|m| m.state == MemberState::Fixed && m.confidence >= 0.7))
    }
}

/// A record that can take part: a bug fix or bug report that may matter beyond its app.
pub fn eligible(c: &Classified) -> bool {
    c.labels.kind == "bug_fix" && c.labels.scope != "app_specific" && c.labels.area != "app_domain" && !c.labels.symptom.trim().is_empty()
}

/// `records` maps app → its classified records; `state_of` says what each record is.
pub fn clusters(records: &[(String, Classified, MemberState)], threshold: f64) -> Vec<Cluster> {
    let pool: Vec<&(String, Classified, MemberState)> = records.iter().filter(|(_, c, _)| eligible(c)).collect();
    let texts: Vec<Vec<String>> = pool.iter().map(|(_, c, _)| tokens(&format!("{} {}", c.labels.symptom, c.labels.root_cause))).collect();
    let vecs = vectors(&texts);
    // Greedy average linkage: take pairs from most to least similar; join their groups only when
    // the two groups are similar on average, so one bridging pair can't chain unrelated bugs.
    let sim = |i: usize, j: usize| if pool[i].1.labels.area == pool[j].1.labels.area { vecs[i].cosine(&vecs[j]) } else { 0.0 };
    let mut edges: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..pool.len() {
        for j in i + 1..pool.len() {
            let s = sim(i, j);
            if s >= threshold {
                edges.push((s, i, j));
            }
        }
    }
    edges.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| (a.1, a.2).cmp(&(b.1, b.2))));
    let mut parent: Vec<usize> = (0..pool.len()).collect();
    let mut members: Vec<Vec<usize>> = (0..pool.len()).map(|i| vec![i]).collect();
    for (_, i, j) in edges {
        let (a, b) = (find(&mut parent, i), find(&mut parent, j));
        if a == b {
            continue;
        }
        let total: f64 = members[a].iter().flat_map(|&x| members[b].iter().map(move |&y| (x, y))).map(|(x, y)| sim(x, y)).sum();
        if total / (members[a].len() * members[b].len()) as f64 >= threshold {
            let (keep, gone) = (a.min(b), a.max(b));
            parent[gone] = keep;
            let moved = std::mem::take(&mut members[gone]);
            members[keep].extend(moved);
        }
    }
    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..pool.len() {
        let root = find(&mut parent, i);
        groups.entry(root).or_default().push(i);
    }
    let mut out: Vec<Cluster> = groups
        .into_values()
        .map(|idx| {
            let mut members: Vec<Member> = idx
                .iter()
                .map(|&i| {
                    let (app, c, state) = pool[i];
                    Member { app: app.clone(), id: c.id.clone(), symptom: c.labels.symptom.clone(), confidence: c.labels.confidence, state: *state }
                })
                .collect();
            members.sort_by(|a, b| a.state.cmp(&b.state).then_with(|| a.id.cmp(&b.id)));
            let fixed_in: BTreeSet<String> = members.iter().filter(|m| m.state == MemberState::Fixed).map(|m| m.app.clone()).collect();
            let open_in: BTreeSet<String> = members.iter().filter(|m| !fixed_in.contains(&m.app)).map(|m| m.app.clone()).collect();
            Cluster {
                area: pool[idx[0]].1.labels.area.clone(),
                members,
                fixed_in: fixed_in.into_iter().collect(),
                open_in: open_in.into_iter().collect(),
            }
        })
        .collect();
    // Most apps first, then most members, then a stable tie-break.
    out.sort_by(|a, b| {
        let apps = |c: &Cluster| c.members.iter().map(|m| m.app.as_str()).collect::<BTreeSet<_>>().len();
        apps(b).cmp(&apps(a)).then_with(|| b.members.len().cmp(&a.members.len())).then_with(|| a.members[0].id.cmp(&b.members[0].id))
    });
    out
}

/// Every classified record of `apps`, with whether it is a merged fix, an open PR or an open issue
/// according to the index now (a PR classified while open may have merged since).
pub fn load(
    index: &crate::index::Index,
    store: &crate::classify::Store,
    apps: &[crate::fix::App],
) -> Result<Vec<(String, Classified, MemberState)>, String> {
    let mut out = Vec::new();
    for app in apps {
        let prs = index.prs(&app.name)?;
        let issues = index.issues(&app.name)?;
        for (id, c) in store.load(&app.name)? {
            let number = |kind: &str| id.strip_prefix(&format!("{}/{kind}/", app.name)).and_then(|n| n.parse::<u64>().ok());
            let state = if let Some(n) = number("pr") {
                match prs.get(&n).map(|p| p.state.as_str()) {
                    Some("MERGED") => MemberState::Fixed,
                    Some("OPEN") => MemberState::OpenPr,
                    _ => continue,
                }
            } else if let Some(n) = number("issue") {
                match issues.get(&n).map(|i| i.state.as_str()) {
                    Some("OPEN") => MemberState::OpenIssue,
                    _ => continue,
                }
            } else {
                continue;
            };
            out.push((app.name.clone(), c, state));
        }
    }
    Ok(out)
}
