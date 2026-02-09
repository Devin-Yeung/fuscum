use num_modular::{ModularCoreOps, ModularPow};

use crate::hash::rolling_hasher::RollingHasher;

pub struct RabinKarp<const B: u64, const M: u64> {
    high_power: u64,
    k: usize,
    hash: u64,
}

impl<const B: u64, const M: u64> RabinKarp<B, M> {
    /// Creates a new Rabin-Karp hasher with the given window size k, base, and modulus.
    pub fn new(k: usize) -> Self {
        let high_power = B.powm((k - 1) as u64, &M);

        Self {
            k,
            high_power,
            hash: 0,
        }
    }
}

impl<const B: u64, const M: u64> RollingHasher for RabinKarp<B, M> {
    type Item = u8;
    type Hash = u64;

    fn window_size(&self) -> usize {
        self.k
    }

    fn hash_window(&mut self, window: &[Self::Item]) -> u64 {
        debug_assert_eq!(window.len(), self.k);
        self.hash = 0;
        for &b in window {
            self.hash = self.hash.mulm(&B, &M).addm(b as u64, &M);
        }
        self.hash
    }

    fn roll(&mut self, leaving: u8, entering: u8) -> u64 {
        // remove = leaving * b^(k-1) mod m
        let remove = (leaving as u64).mulm(self.high_power, &M);

        // remove leaving byte
        self.hash = self.hash.subm(remove, &M);

        // add entering byte
        self.hash = self
            .hash
            .mulm(B, &M) // left shift
            .addm(entering as u64, &M); // add entering byte

        self.hash
    }

    fn reset(&mut self) {
        self.hash = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::{rabin_karp::RabinKarp, rolling_hasher::RollingHashIter};

    #[test]
    fn it_works() {
        // M = 2^64, base = 257, k = 5
        let hasher: RabinKarp<257, { u64::MAX }> = super::RabinKarp::new(5);
        let data = b"hello world";
        let iter = RollingHashIter::new(data, hasher);

        let expect = vec![
            455418516756u64,
            442449928396,
            472987399134,
            473033127393,
            484785289959,
            141626390244,
            521025714984,
        ];
        let got: Vec<_> = iter.map(|(_, v)| v).collect();
        assert_eq!(expect, got);
    }
}
