use crate::kgram::Kgram;
use crate::preprocess::Preprocessor;
use crate::winnow::winnowing;
use std::collections::HashSet;

pub trait WithFingerprint {
    type Hash: std::hash::Hash + Eq;
    /// The fingerprint of the object, which is a set of hashes
    fn fingerprint(&self) -> HashSet<Self::Hash>;

    /// Calculate the similarity between two objects
    /// The similarity is calculated as the intersection of the hashes over the base
    fn similarity<S: WithFingerprint<Hash = Self::Hash>>(&self, against: &S) -> f32 {
        let base = self.fingerprint();
        let against = against.fingerprint();

        let intersection = base.intersection(&against).count() as f32;
        intersection / base.len() as f32
    }
}

pub struct FingerPrintConfig {
    pub k: usize,
    pub window_size: usize,
}

impl FingerPrintConfig {
    pub fn new(k: usize, window_size: usize) -> Self {
        Self { k, window_size }
    }
}

impl Default for FingerPrintConfig {
    fn default() -> Self {
        Self {
            k: 35,
            window_size: 40,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct FingerPrint {
    raw_fingerprint: Vec<(u64, usize)>,
}

pub struct FingerPrintGenerator<P: Preprocessor, K: Kgram> {
    pub config: FingerPrintConfig,
    pub preprocessor: P,
    pub kgram: K,
}

impl<P: Preprocessor, K: Kgram> FingerPrintGenerator<P, K> {
    pub fn generate<S: AsRef<str>>(&self, src: S) -> FingerPrint {
        let preprocessed = self.preprocessor.preprocess(src.as_ref());
        let k_grams = self.kgram.k_gram(preprocessed.as_bytes(), self.config.k);
        let fingerprints = winnowing(k_grams, self.config.window_size);
        FingerPrint {
            raw_fingerprint: fingerprints,
        }
    }
}

impl FingerPrint {
    /// Return the fingerprint as a set of hashes, without their positions
    pub fn fingerprint(&self) -> HashSet<u64> {
        self.raw_fingerprint.iter().map(|(hash, _)| *hash).collect()
    }

    /// Return the raw fingerprints with their positions
    pub fn raw_fingerprint(&self) -> &[(u64, usize)] {
        &self.raw_fingerprint
    }
}

impl WithFingerprint for FingerPrint {
    type Hash = u64;

    fn fingerprint(&self) -> HashSet<Self::Hash> {
        self.fingerprint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kgram::default_rolling_kgram, preprocess::PythonPreprocessor};

    #[test]
    fn it_works() {
        let src = "def f(a, b, c):\n\ta = 1";
        let gen = FingerPrintGenerator {
            config: FingerPrintConfig::new(3, 3),
            preprocessor: PythonPreprocessor::default(),
            kgram: default_rolling_kgram(),
        };

        let fp = gen.generate(src);
        insta::assert_debug_snapshot!(fp.raw_fingerprint());
    }
}
