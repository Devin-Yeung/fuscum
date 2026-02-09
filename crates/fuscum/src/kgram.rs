use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::hash::rabin_karp::RabinKarp;
use crate::hash::rolling_hasher::RollingHashIter;

pub trait Kgram {
    fn k_gram(&self, data: &[u8], k: usize) -> Vec<u64>;
}

pub struct StdHashKgram;

impl Kgram for StdHashKgram {
    fn k_gram(&self, data: &[u8], k: usize) -> Vec<u64> {
        let mut hashes = Vec::new();

        if k > data.len() {
            return hashes;
        }

        for window in data.windows(k) {
            let mut hasher = DefaultHasher::new();
            window.hash(&mut hasher);
            hashes.push(hasher.finish());
        }

        hashes
    }
}

pub struct RollingHashKgram {
    base: u64,
    modulus: u64,
}

impl Default for RollingHashKgram {
    fn default() -> Self {
        Self {
            base: 257,
            modulus: u64::MAX,
        }
    }
}

impl Kgram for RollingHashKgram {
    fn k_gram(&self, data: &[u8], k: usize) -> Vec<u64> {
        let hasher = RabinKarp::new(k, self.base, self.modulus);
        RollingHashIter::new(data, hasher)
            .map(|(_, hash)| hash)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::kgram::{Kgram, RollingHashKgram, StdHashKgram};

    #[test]
    fn std_k_gram() {
        let text = "adorunrunrunadorunrun";
        let k = 5;
        let result = StdHashKgram.k_gram(&text.as_bytes(), k);
        insta::assert_debug_snapshot!(result);
    }

    #[test]
    fn rolling_k_gram() {
        let text = "adorunrunrunadorunrun";
        let k = 5;
        let result = RollingHashKgram::default().k_gram(&text.as_bytes(), k);
        insta::assert_debug_snapshot!(result);
    }
}
