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
    entries: usize,
    buckets: Vec<BucketEntry<K, V>>,
}

const BUCKET_LOAD_FACTOR_MAX: f64 = 1.0;
const BUCKET_RESIZE_LOAD_FACTOR: f64 = 0.5;

impl<K: BucketKeyReq, V> BucketHashmap<K, V> {
    /// (std) Create a new hashmap.
    pub fn new() -> Self {
        Self {
            buckets: vec![Default::default()],
            entries: 0,
        }
    }

    /// (std) Construct a hashmap with at least this capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let bucket_count = (capacity as f64 * BUCKET_LOAD_FACTOR_MAX).ceil() as usize;
        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(BucketEntry::default());
        }
        Self {
            buckets,
            entries: 0,
        }
    }

    fn calculate_bucket_index(&self, k: &K) -> usize {
        // First calculate the hash.
        let mut hasher = std::hash::DefaultHasher::new();
        k.hash(&mut hasher);
        let h = hasher.finish();
        h.rem_euclid(self.buckets.len() as u64) as usize
    }

    /// (std) Reserves at least this additional size.
    pub fn reserve(&mut self, additional: usize) {
        self.resize_to(self.entries + additional);
    }

    fn resize_to(&mut self, new_entries: usize) {
        let new_factor = new_entries as f64 / self.buckets.len() as f64;
        if new_factor < BUCKET_LOAD_FACTOR_MAX {
            return; // no work to do.
        }
        let new_size = (new_entries as f64 * (1.0 / BUCKET_RESIZE_LOAD_FACTOR)).ceil();
        let new_size = new_size as usize;

        let mut new_map = Self::with_capacity(new_size);

        // Drain self into the new map.
        for mut v in self.buckets.drain(..) {
            for (k, v) in v.entries.drain(..) {
                new_map.insert(k, v);
            }
        }

        println!("got resize event to {new_size}");

        // Replace the map.
        *self = new_map;
    }

    /// (std) Insert a key.
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
        self.entries += 1;
        b.entries.push((key, value));

        // Resize if that was actually necessary
        self.resize_to(self.entries);
    }

    /// (std) Check if a key exists.
    pub fn contains_key(&self, k: &K) -> bool {
        let bucket_index = self.calculate_bucket_index(k);
        // We found the bucket.
        let b: &_ = &self.buckets[bucket_index];
        // Search in that bucket.
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
        for i in 0..32 {
            h.insert(i, i);
        }
    }
}
