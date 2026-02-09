use crate::kgram::Kgram;
use crate::preprocess::Preprocessor;
use crate::winnow::winnowing;
use std::collections::HashSet;

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
    fingerprints: Vec<(u64, usize)>,
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
        FingerPrint { fingerprints }
    }
}

impl FingerPrint {
    pub fn similarity(&self, other: &Self) -> f32 {
        let cur = self.hashes();
        let other = other.hashes();

        let intersection = cur.intersection(&other).count() as f32;
        intersection / cur.len() as f32
    }

    pub fn hashes(&self) -> HashSet<u64> {
        self.fingerprints.iter().map(|(hash, _)| *hash).collect()
    }

    pub fn fingerprints(&self) -> &[(u64, usize)] {
        &self.fingerprints
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
        insta::assert_debug_snapshot!(fp.fingerprints());
    }
}
