use std::hash::{Hash, Hasher};

pub trait BucketKeyReq: Hash + Eq {}
impl<T: Hash + Eq> BucketKeyReq for T {}

#[derive(Debug, Default)]
pub struct BucketHashmap<K: BucketKeyReq, V> {
    entries: usize,
    buckets: Vec<Vec<(K, V)>>,
}

const BUCKET_LOAD_FACTOR_MAX: f64 = 1.0;
const BUCKET_RESIZE_LOAD_FACTOR: f64 = 0.5;

impl<K: BucketKeyReq, V> BucketHashmap<K, V> {
    fn calculate_bucket_index(&self, k: &K) -> usize {
        // First calculate the hash.
        let mut hasher = std::hash::DefaultHasher::new();
        k.hash(&mut hasher);
        let h = hasher.finish();
        h.rem_euclid(self.buckets.len() as u64) as usize
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
            for (k, v) in v.drain(..) {
                new_map.insert(k, v);
            }
        }

        println!("got resize event to {new_size}");

        // Replace the map.
        *self = new_map;
    }

    pub fn debug_info(&self) {
        let load = self.entries as f64 / self.buckets.len() as f64;
        println!(" load: {load}");
        println!(" buckets: {}", self.buckets.len());
        println!(" entries: {}", self.entries);
        for (i, b) in self.buckets.iter().enumerate() {
            println!(" b[{i}]: {}", b.len());
        }
    }
}

// Use this block to hold the 'std' methods.
impl<K: BucketKeyReq, V> BucketHashmap<K, V> {
    /// Create a new hashmap.
    pub fn new() -> Self {
        Self {
            buckets: vec![Default::default()],
            entries: 0,
        }
    }

    /// Construct a hashmap with at least this capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let bucket_count = (capacity as f64 * BUCKET_LOAD_FACTOR_MAX).ceil() as usize;
        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(Default::default());
        }
        Self {
            buckets,
            entries: 0,
        }
    }

    /// Reserves at least this additional size.
    pub fn reserve(&mut self, additional: usize) {
        self.resize_to(self.entries + additional);
    }

    /// Insert a key.
    pub fn insert(&mut self, key: K, value: V) {
        let bucket_index = self.calculate_bucket_index(&key);
        // We found the bucket.
        let b: &mut _ = &mut self.buckets[bucket_index];
        // In that bucket we may already have the key, so search for it if so update it.
        for (bk, bv) in b.iter_mut() {
            if *bk == key {
                *bv = value;
                return;
            }
        }

        // We did not find this key already, so we append it to the bucket.
        self.entries += 1;
        b.push((key, value));

        // Resize if that was actually necessary
        self.resize_to(self.entries);
    }

    /// Check if a key exists.
    pub fn contains_key(&self, k: &K) -> bool {
        let bucket_index = self.calculate_bucket_index(k);
        // We found the bucket.
        let b: &_ = &self.buckets[bucket_index];
        // Search in that bucket.
        for (bk, _bv) in b.iter() {
            if *bk == *k {
                return true;
            }
        }
        false
    }

    /// Return current number of entries in the map.
    pub fn len(&self) -> usize {
        self.entries
    }

    /// Remove an entry from the hashmap.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.calculate_bucket_index(&key);
        if let Some(index_in_bucket) = self.buckets[bucket_index]
            .iter()
            .position(|(bk, _)| *bk == *key)
        {
            let v = self.buckets[bucket_index].swap_remove(index_in_bucket);
            self.entries -= 1;
            Some(v.1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bucket_insert() {
        let mut h = BucketHashmap::<u64, u64>::new();
        h.insert(300, 3);
        h.insert(500, 8);
        assert!(h.contains_key(&300));
        assert!(h.contains_key(&500));
        assert!(!h.contains_key(&8));
        for i in 0..32 {
            h.insert(i, i);
            println!("h size: {}", h.len());
            // h.debug_info();
        }
        assert_eq!(h.len(), 34);
        assert!(h.remove(&300).is_some());
        assert_eq!(h.len(), 33);
        assert!(!h.contains_key(&300));
    }
}
