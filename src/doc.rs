use crate::fingerprint::FingerPrint;
use std::cmp::Ordering;
use std::collections::HashSet;

pub trait Similar {
    type Hash: std::hash::Hash + Eq;
    fn hashes(&self) -> HashSet<Self::Hash>;

    /// Calculate the similarity between two objects
    /// The similarity is calculated as the intersection of the hashes over the base
    fn similarity<S: Similar<Hash = Self::Hash>>(&self, against: &S) -> f32 {
        let base = self.hashes();
        let against = against.hashes();

        let intersection = base.intersection(&against).count() as f32;
        intersection / base.len() as f32
    }
}

#[derive(Eq, PartialEq)]
pub struct Doc {
    pub(crate) name: String,
    pub(crate) finger_print: FingerPrint,
}

impl Similar for Doc {
    type Hash = u64;

    fn hashes(&self) -> HashSet<Self::Hash> {
        self.finger_print.hashes()
    }
}

impl Doc {
    pub fn new(name: String, finger_print: FingerPrint) -> Self {
        Self { name, finger_print }
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

pub struct MultiDoc {
    name: String,
    docs: Vec<Doc>,
}

impl Similar for MultiDoc {
    type Hash = u64;

    fn hashes(&self) -> HashSet<Self::Hash> {
        self.hashes()
    }
}

pub struct MultiDocSimilarity {
    base: String,
    against: String,
    score: f32,
    sources: Vec<(String, f32)>,
}

impl MultiDoc {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            docs: Vec::new(),
        }
    }

    pub fn add_doc(&mut self, doc: Doc) {
        self.docs.push(doc);
    }

    /// Return all the hashes from all the documents
    pub fn hashes(&self) -> HashSet<u64> {
        self.docs
            .iter()
            .map(|doc| doc.finger_print.hashes())
            .flatten()
            .collect()
    }

    pub fn docs(&self) -> &[Doc] {
        &self.docs
    }

    pub fn similarity(&self, other: &Self) -> MultiDocSimilarity {
        let sources: Vec<_> = other
            .docs
            .iter()
            .map(|against| {
                let score = Similar::similarity(self, against);
                (against.name.clone(), score)
            })
            .collect();

        let score = Similar::similarity(self, other);

        MultiDocSimilarity {
            base: self.name.clone(),
            against: other.name.clone(),
            score,
            sources,
        }
    }
}
