use crate::preprocess::Preprocessor;
use crate::winnow::{k_gram, winnowing};
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

impl FingerPrint {
    pub fn new<P, S>(src: S, pp: P, config: FingerPrintConfig) -> Self
    where
        P: Preprocessor,
        S: AsRef<str>,
    {
        let src = pp.preprocess(&src);
        let hashes = k_gram(&src, config.k);
        let fingerprints = winnowing(&hashes, config.window_size);
        Self { fingerprints }
    }

    pub fn similarity(&self, other: &Self) -> f32 {
        let cur = self.hashes();
        let other = other.hashes();

        let intersection = cur.intersection(&other).count() as f32;
        intersection / cur.len() as f32
    }

    fn hashes(&self) -> HashSet<u64> {
        self.fingerprints.iter().map(|(hash, _)| *hash).collect()
    }

    pub fn fingerprints(&self) -> &[(u64, usize)] {
        &self.fingerprints
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preprocess::python::PythonPreprocessor;

    #[test]
    fn it_works() {
        let src = "def f(a, b, c):\n\ta = 1";
        let fp = FingerPrint::new(src, PythonPreprocessor, FingerPrintConfig::new(3, 3));
        insta::assert_debug_snapshot!(fp.fingerprints());
    }
}
