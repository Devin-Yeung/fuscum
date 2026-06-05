/// Perform winnowing on a sequence of hash values, returning the selected fingerprints and their positions.
pub fn winnowing<T>(hashes: T, window_size: usize, robust: bool) -> Vec<(u64, usize)>
where
    T: AsRef<[u64]>,
{
    let seq = hashes.as_ref();
    let mut finger_prints = Vec::new();

    for i in 0..seq.len().saturating_sub(window_size) {
        let window = &seq[i..i + window_size];
        let (min_hash, min_idx) = rightmost_minimal(window);
        let idx = i + min_idx;
        // Only store a hash if it's not the same as in the previous window.
        // In the robust variant of the winnowing algorithm, also reject hashes
        // from the previous window which have the same value, if they still belong
        // to the current window. This breaks the locality of hash selection but helps
        // reduce fingerprint size on low-entropy strings.
        if finger_prints
            .last()
            .is_none_or(|(previous_hash, previous_position)| {
                *previous_position != idx
                    && !(robust && *previous_hash == min_hash && *previous_position >= i)
            })
        {
            finger_prints.push((min_hash, idx));
        }
    }

    finger_prints
}

/// Find the rightmost minimal hash value in the window
fn rightmost_minimal(window: &[u64]) -> (u64, usize) {
    let mut min = u64::MAX;
    let mut idx = 0usize;
    for (i, &hash) in window.iter().enumerate() {
        if hash <= min {
            min = hash;
            idx = i;
        }
    }
    (min, idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rightmost_minimal_works() {
        assert_eq!(rightmost_minimal(&[3, 2, 5, 2]), (2, 3));
    }

    #[test]
    fn it_works() {
        let hashes = [
            77u64, 74, 42, 17, 98, 50, 17, 98, 8, 88, 67, 39, 77, 74, 42, 17, 98,
        ];
        let window_size = 4;
        assert_eq!(
            super::winnowing(hashes, window_size, false),
            // [(17, 3), (17, 6), (8, 8), (39, 11), (17, 15)]
            vec![(17, 3), (17, 6), (8, 8), (39, 11), (17, 15)]
        );
    }

    #[test]
    fn low_entropy_strings() {
        let hashes = [
            77u64, 74, 42, 17, 98, 50, 50, 50, 50, 50, 50, 50, 50, 50, 8, 88, 67, 39, 77,
        ];
        let window_size = 4;
        // the standard version of winnowing selects many consecutive hashes
        assert_eq!(
            super::winnowing(hashes, window_size, false),
            vec![
                (17, 3),
                (50, 7),
                (50, 8),
                (50, 9),
                (50, 10),
                (50, 11),
                (50, 12),
                (50, 13),
                (8, 14)
            ]
        );
        // the robust version avoids that by reusing hashes when they are still in the window
        assert_eq!(
            super::winnowing(hashes, window_size, true),
            vec![(17, 3), (50, 7), (50, 11), (8, 14)]
        );
    }
}
