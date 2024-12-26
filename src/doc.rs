use crate::fingerprint::FingerPrint;

#[derive(Eq, PartialEq)]
pub struct Doc {
    pub(crate) name: String,
    pub(crate) finger_print: FingerPrint,
}

#[derive(Debug)]
pub struct Similarity {
    base: String,
    against: String,
    pub(crate) score: f32,
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
