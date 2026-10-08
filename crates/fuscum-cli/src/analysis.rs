use anyhow::Result;
use rayon::prelude::*;

use crate::discovery::Submission;
use crate::summary::{PairSummary, Summaries, Summary};

/// A submission with its fingerprint pre-processed for fast pairwise comparison.
struct Prepared<'a> {
    name: &'a str,
    /// Sorted, deduplicated fingerprint hashes
    hashes: Vec<u64>,
}

/// Same definition as `WithFingerprint::similarity`: |base ∩ other| / |base|,
/// computed by merging two sorted, deduplicated slices.
fn similarity(base: &[u64], other: &[u64]) -> f32 {
    if base.is_empty() {
        return 0.0;
    }
    let (mut i, mut j, mut common) = (0, 0, 0usize);
    while i < base.len() && j < other.len() {
        match base[i].cmp(&other[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                common += 1;
                i += 1;
                j += 1;
            }
        }
    }
    common as f32 / base.len() as f32
}

pub struct SimilarityAnalyzer {
    threshold: f32,
    top_k: usize,
}

impl SimilarityAnalyzer {
    pub fn new(threshold: f32, top_k: usize) -> Self {
        Self { threshold, top_k }
    }

    pub fn analyze_fingerprints(&self, submissions: &[Submission]) -> Result<Summaries> {
        // Build each fingerprint's sorted, deduplicated hash list once, so every
        // pairwise comparison is a linear merge instead of rebuilding HashSets.
        let prepared: Vec<Prepared> = submissions
            .iter()
            .map(|s| {
                let mut hashes: Vec<u64> = s.fingerprint.fingerprint().into_iter().collect();
                hashes.sort_unstable();
                Prepared {
                    name: &s.name,
                    hashes,
                }
            })
            .collect();

        let mut summaries: Vec<Summary> = prepared
            .par_iter()
            .map(|base| {
                let pairs = self.compute_pair_summaries(base, &prepared);
                let max_score = pairs.first().map(|p| p.score).unwrap_or(0.0);

                Summary {
                    base: base.name.to_string(),
                    max_score,
                    against: pairs,
                }
            })
            .collect();

        summaries.sort_by(|a, b| b.max_score.partial_cmp(&a.max_score).unwrap());
        summaries.retain(|s| s.max_score >= self.threshold);

        Ok(summaries.into())
    }

    fn compute_pair_summaries(&self, base: &Prepared, others: &[Prepared]) -> Vec<PairSummary> {
        let mut pairs: Vec<PairSummary> = others
            .iter()
            .filter(|other| other.name != base.name)
            .map(|other| PairSummary {
                against: other.name.to_string(),
                score: similarity(&base.hashes, &other.hashes),
            })
            .collect();

        pairs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        pairs.truncate(self.top_k);
        pairs
    }
}
