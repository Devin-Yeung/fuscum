use serde::Serialize;

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
