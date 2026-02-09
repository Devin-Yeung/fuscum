/// A trait for rolling hash functions that compute hash values over sliding windows.
///
/// Rolling hash functions are designed to efficiently compute hash values for consecutive
/// windows of data by incrementally updating the hash as the window slides. Instead of
/// recomputing the entire hash from scratch for each window position, rolling hash functions
/// update the hash by removing the contribution of the element leaving the window and adding
/// the contribution of the element entering the window.
///
/// This trait is particularly useful for algorithms like:
/// - Rabin-Karp string matching
/// - Content-defined chunking (CDC)
/// - Deduplication algorithms
///
/// # Example
///
/// ```ignore
/// use fuscum::hash::RollingHasher;
///
/// fn find_matches<H: RollingHasher>(hasher: &mut H, data: &[H::Item]) {
///     let window_size = hasher.window_size();
///     
///     // Hash the first window
///     let mut hash = hasher.hash_window(&data[..window_size]);
///     
///     // Roll through remaining data
///     for i in window_size..data.len() {
///         hash = hasher.roll(data[i - window_size], data[i]);
///         // Process hash...
///     }
/// }
/// ```
pub trait RollingHasher {
    /// The type of items being hashed (typically `u8` for byte data).
    type Item: Copy;

    /// The hash value type returned by the hasher.
    type Hash: Copy + Eq;

    /// Returns the size of the sliding window in number of items.
    ///
    /// This determines how many consecutive items are included in each hash computation.
    fn window_size(&self) -> usize;

    /// Computes the hash of a complete window from scratch.
    ///
    /// This method is typically used for the initial window or after a reset.
    /// The window slice must have a length equal to `window_size()`.
    ///
    /// # Arguments
    ///
    /// * `window` - A slice of items with length equal to `window_size()`
    ///
    /// # Returns
    ///
    /// The hash value for the given window.
    fn hash_window(&mut self, window: &[Self::Item]) -> Self::Hash;

    /// Incrementally updates the hash by rolling the window one position forward.
    ///
    /// This method efficiently computes the hash of the new window by:
    /// 1. Removing the contribution of the `leaving` item (exiting the window)
    /// 2. Adding the contribution of the `entering` item (entering the window)
    ///
    /// # Arguments
    ///
    /// * `leaving` - The item exiting the window (leftmost item of the previous window)
    /// * `entering` - The item entering the window (new rightmost item)
    ///
    /// # Returns
    ///
    /// The hash value for the new window position.
    fn roll(&mut self, leaving: Self::Item, entering: Self::Item) -> Self::Hash;

    /// Resets the hasher to its initial state.
    ///
    /// After calling this method, the hasher should behave as if it were newly created,
    /// forgetting any previous window state.
    fn reset(&mut self);
}

pub struct RollingHashIter<'a, H: RollingHasher<Item = u8>> {
    data: &'a [u8],
    hasher: H,
    pos: usize,
    k: usize,
    initialized: bool,
}

impl<'a, H: RollingHasher<Item = u8>> RollingHashIter<'a, H> {
    pub fn new(data: &'a [u8], hasher: H) -> Self {
        let k = hasher.window_size();
        Self {
            data,
            hasher,
            pos: 0,
            k,
            initialized: false,
        }
    }
}

impl<'a, H: RollingHasher<Item = u8>> Iterator for RollingHashIter<'a, H> {
    type Item = (usize, H::Hash);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initialized {
            // cold start
            if self.data.len() < self.k {
                return None;
            }
            let hash = self.hasher.hash_window(&self.data[..self.k]);
            self.pos = self.k;
            self.initialized = true;
            return Some((0, hash));
        }

        // rolling
        if self.pos >= self.data.len() {
            return None;
        }

        let leaving = self.data[self.pos - self.k];
        let entering = self.data[self.pos];
        let start = self.pos - self.k + 1;

        let hash = self.hasher.roll(leaving, entering);
        self.pos += 1;

        Some((start, hash))
    }
}
