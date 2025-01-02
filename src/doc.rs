use crate::fingerprint::FingerPrint;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct Doc {
    pub(crate) name: String,
    pub(crate) finger_print: FingerPrint,
}

impl Doc {
    pub fn new (name: String, finger_print: FingerPrint) -> Self {
        Self {
            name,
            finger_print,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Similarity {
    base: String,
    against: String,
    score: f32,
}

impl PartialOrd for Similarity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Similarity {
    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn against(&self) -> &str {
        &self.against
    }
}

impl Doc {
    pub fn similarity(&self, other: &Self) -> Similarity {
        let score = self.finger_print.similarity(&other.finger_print);
        Similarity {
            base: self.name.clone(),
            against: other.name.clone(),
            score,
        }
    }
}
