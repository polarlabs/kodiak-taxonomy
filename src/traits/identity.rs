use std::hash::Hash;

/// Elements need an identity K with K: Hash + Eq
pub trait Identity<K>
where
    K: Hash + Eq,
{
    fn id(&self) -> K;
}
