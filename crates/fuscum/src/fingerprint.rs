use typed_builder::TypedBuilder;

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

#[derive(TypedBuilder)]
pub struct FingerPrintConfig {
    /// the size of the k-grams to hash
    #[builder(default = 35)]
    pub k: usize,
    /// the number of consecutive hashes to consider in one window
    #[builder(default = 40)]
    pub window_size: usize,
    /// whether to use robust winnowing, a variant of the winnowing algorithm
    /// which reduces fingerprint size when long low-entropy sequences are present
    #[builder(default = false)]
    pub robust: bool,
}

impl Default for FingerPrintConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Eq, PartialEq)]
pub struct FingerPrint {
    raw_fingerprint: Vec<(u64, usize)>,
}

pub struct FingerPrintGenerator<P: Preprocessor> {
    pub config: FingerPrintConfig,
    pub preprocessor: P,
    pub kgram: Box<dyn Kgram>,
}

impl<P: Preprocessor> FingerPrintGenerator<P> {
    pub fn generate<S: AsRef<str>>(&self, src: S) -> FingerPrint {
        let preprocessed = self.preprocessor.preprocess(src.as_ref());
        let k_grams = self.kgram.k_gram(preprocessed.as_bytes(), self.config.k);
        let fingerprints = winnowing(k_grams, self.config.window_size, self.config.robust);
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
    use crate::kgram::default_rolling_kgram;
    use crate::preprocess::RegexPreprocessor;

    #[cfg(feature = "ast")]
    #[test]
    fn it_works() {
        use crate::preprocess::PythonPreprocessor;

        let src = "def f(a, b, c):\n\ta = 1";
        let gen = FingerPrintGenerator {
            config: FingerPrintConfig::builder().k(3).window_size(3).build(),
            preprocessor: PythonPreprocessor::default(),
            kgram: Box::new(default_rolling_kgram()),
        };

        let fp = gen.generate(src);
        insta::assert_debug_snapshot!(fp.raw_fingerprint());
    }

    #[test]
    fn with_text_preprocessor() {
        let src = "how much wood could a woodchuck chuck";
        let gen = FingerPrintGenerator {
            config: FingerPrintConfig::builder().k(3).window_size(3).build(),
            preprocessor: RegexPreprocessor::whitespace(),
            kgram: Box::new(default_rolling_kgram()),
        };

        let fp = gen.generate(src);
        insta::assert_debug_snapshot!(fp.raw_fingerprint());
    }
}
