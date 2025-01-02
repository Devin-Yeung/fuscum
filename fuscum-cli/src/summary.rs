use fuscum::doc::MultiDocSimilarity;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Summary {
    pub(crate) base: String,
    pub(crate) max_score: f32,
    pub(crate) against: Vec<SourceSummary>,
}

#[derive(Debug, Serialize)]
pub struct SourceSummary {
    pub(crate) against: String,
    pub(crate) score: f32,
    pub(crate) sources: Vec<(String, f32)>,
}

impl Summary {
    pub fn score(&self) -> f32 {
        self.against
            .iter()
            .map(|s| s.score)
            .reduce(f32::max)
            .expect("should have max")
    }
}

impl From<MultiDocSimilarity> for SourceSummary {
    fn from(similarity: MultiDocSimilarity) -> Self {
        Self {
            against: similarity.against().to_string(),
            score: similarity.score(),
            sources: similarity
                .top_n_sources(5)
                .iter()
                .map(|(name, score)| (name.to_string(), *score))
                .collect(),
        }
    }
}
