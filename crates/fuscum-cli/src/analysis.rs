use anyhow::Result;
use fuscum::fingerprint::WithFingerprint;
use rayon::prelude::*;

use crate::discovery::Submission;
use crate::summary::{PairSummary, Summaries, Summary};

pub struct SimilarityAnalyzer {
    threshold: f32,
    top_k: usize,
}

impl SimilarityAnalyzer {
    pub fn new(threshold: f32, top_k: usize) -> Self {
        Self { threshold, top_k }
    }

    pub fn analyze_fingerprints(&self, submissions: &[Submission]) -> Result<Summaries> {
        let mut summaries: Vec<Summary> = submissions
            .par_iter()
            .map(|base| {
                let pairs = self.compute_pair_summaries(base, submissions);
                let max_score = pairs.first().map(|p| p.score).unwrap_or(0.0);

                Summary {
                    base: base.name.clone(),
                    max_score,
                    against: pairs,
                }
            })
            .collect();

        summaries.sort_by(|a, b| b.max_score.partial_cmp(&a.max_score).unwrap());
        summaries.retain(|s| s.max_score >= self.threshold);

        Ok(summaries.into())
    }

    fn compute_pair_summaries(&self, base: &Submission, others: &[Submission]) -> Vec<PairSummary> {
        let mut pairs: Vec<PairSummary> = others
            .iter()
            .filter(|other| other.name != base.name)
            .map(|other| PairSummary {
                against: other.name.clone(),
                score: base.fingerprint.similarity(&other.fingerprint),
            })
            .collect();

        pairs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        pairs.truncate(self.top_k);
        pairs
    }
}
