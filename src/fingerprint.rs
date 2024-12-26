use crate::preprocess::Preprocessor;
use crate::winnow::{k_gram, winnowing};

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
            window_size: 5,
        }
    }
}

pub struct FingerPrint {
    fingerprints: Vec<(u64, usize)>,
}

impl FingerPrint {
    pub fn new<P, S>(src: S, config: FingerPrintConfig) -> Self
    where
        P: Preprocessor,
        S: AsRef<str>,
    {
        let src = P::preprocess(&src);
        let hashes = k_gram(&src, config.k);
        let fingerprints = winnowing(&hashes, config.window_size);
        Self { fingerprints }
    }

    pub fn fingerprints(&self) -> &[(u64, usize)] {
        &self.fingerprints
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preprocess::python;

    #[test]
    fn it_works() {
        let src = "def f(a, b, c):\n\ta = 1";
        let fp =
            FingerPrint::new::<python::PythonPreprocessor, _>(src, FingerPrintConfig::new(3, 3));
        insta::assert_debug_snapshot!(fp.fingerprints());
    }
}
