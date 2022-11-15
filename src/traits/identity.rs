use std::hash::Hash;

/// Elements need an identity of type `K` with `K: Hash + Eq`.
pub trait Identity<K: Hash + Eq> {
    /// Returns a unique identifier for an item. Items with the same id are considered as identical by the taxonomy.
    // Rust API Guideline - C-OBJECT
    fn id(&self) -> K
    where
        Self: Sized;
}
