pub fn winnowing<T>(hashes: T, window_size: usize) -> Vec<(u64, usize)>
where
    T: AsRef<[u64]>,
{
    let seq = hashes.as_ref();
    let mut finger_prints = Vec::new();
    // the global position of the minimum hash value in the *previous* window
    let mut idx = 0usize;

    for i in 0..seq.len().saturating_sub(window_size) {
        let window = &seq[i..i + window_size];
        let (min_hash, min_idx) = rightmost_minimal(window);
        if idx != i + min_idx {
            // make sure it's not the same as the last window
            idx = i + min_idx;
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
        if hash < min {
            min = hash;
            idx = i;
        }
    }
    (min, idx)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let hashes = [
            77u64, 74, 42, 17, 98, 50, 17, 98, 8, 88, 67, 39, 77, 74, 42, 17, 98,
        ];
        let window_size = 4;
        assert_eq!(
            super::winnowing(&hashes, window_size),
            // [(17, 3), (17, 6), (8, 8), (39, 11), (17, 15)]
            vec![(17, 3), (17, 6), (8, 8), (39, 11), (17, 15)]
        );
    }
}
