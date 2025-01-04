use std::hash::{Hash, Hasher};
// use std::borrow::Borrow;

pub trait BucketKeyReq: Hash + Eq {}
impl<T: Hash + Eq> BucketKeyReq for T {}

#[derive(Debug)]
struct BucketEntry<K, V> {
    entries: Vec<(K, V)>,
}
impl<K, V> Default for BucketEntry<K, V> {
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

#[derive(Debug, Default)]
pub struct BucketHashmap<K: BucketKeyReq, V> {
    buckets: Vec<BucketEntry<K, V>>,
}

const BUCKET_DEFAULT_CAPACITY: usize = 8;

impl<K: BucketKeyReq, V> BucketHashmap<K, V> {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(BUCKET_DEFAULT_CAPACITY);
        for _ in 0..BUCKET_DEFAULT_CAPACITY {
            buckets.push(BucketEntry::default());
        }
        Self { buckets }
    }

    fn calculate_bucket_index(&self, k: &K) -> usize {
        // First calculate the hash.
        let mut hasher = std::hash::DefaultHasher::new();
        k.hash(&mut hasher);
        let h = hasher.finish();
        h.rem_euclid(self.buckets.len() as u64) as usize
    }

    /// Insert a key and value pair into the map.
    pub fn insert(&mut self, key: K, value: V) {
        let bucket_index = self.calculate_bucket_index(&key);
        // We found the bucket.
        let b: &mut _ = &mut self.buckets[bucket_index];
        // In that bucket we may already have the key, so search for it if so update it.
        for (bk, bv) in b.entries.iter_mut() {
            if *bk == key {
                *bv = value;
                return;
            }
        }
        // We did not find this key already, so we append it to the bucket.
        b.entries.push((key, value));
    }

    pub fn contains_key(&self, k: &K) -> bool {
        let bucket_index = self.calculate_bucket_index(k);
        // We found the bucket.
        let b: &_ = &self.buckets[bucket_index];
        // In that bucket we may already have the key, so search for it if so update it.
        for (bk, _bv) in b.entries.iter() {
            if *bk == *k {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bucket_insert() {
        let mut h = BucketHashmap::<u64, u64>::new();
        h.insert(3, 3);
        h.insert(5, 8);
        assert!(h.contains_key(&3));
        assert!(h.contains_key(&5));
        assert!(!h.contains_key(&8));
    }
}
