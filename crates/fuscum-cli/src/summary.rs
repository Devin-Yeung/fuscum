use serde::Serialize;
use std::ops::Deref;

#[derive(Debug, Serialize)]
pub struct Summary {
    pub base: String,
    pub max_score: f32,
    pub against: Vec<PairSummary>,
}

#[derive(Debug, Serialize)]
pub struct PairSummary {
    pub against: String,
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct Summaries(pub Vec<Summary>);

impl Deref for Summaries {
    type Target = Vec<Summary>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Summary>> for Summaries {
    fn from(vec: Vec<Summary>) -> Self {
        Self(vec)
    }
}
