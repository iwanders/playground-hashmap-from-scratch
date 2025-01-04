use std::hash::{Hash, Hasher};

pub trait BucketKeyReq: Hash + Eq {}
impl<T: Hash + Eq> BucketKeyReq for T {}

const BUCKET_LOAD_FACTOR_MAX: f64 = 1.0;
const BUCKET_RESIZE_LOAD_FACTOR: f64 = 0.5;

#[derive(Debug)]
pub struct BucketHashmap<K: BucketKeyReq, V> {
    entries: usize,
    load_factor_max: f64,
    resize_load_factor: f64,
    buckets: Vec<Vec<(K, V)>>,
}
impl<K: BucketKeyReq, V> Default for BucketHashmap<K, V> {
    fn default() -> Self {
        Self {
            entries: 0,
            load_factor_max: BUCKET_LOAD_FACTOR_MAX,
            resize_load_factor: BUCKET_RESIZE_LOAD_FACTOR,
            buckets: vec![Default::default()],
        }
    }
}
impl<K: BucketKeyReq + Clone, V: Clone> Clone for BucketHashmap<K, V> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries,
            load_factor_max: self.load_factor_max,
            resize_load_factor: self.resize_load_factor,
            buckets: self.buckets.clone(),
        }
    }
}

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
        if new_factor < self.load_factor_max {
            return; // no work to do.
        }
        let new_size = (new_entries as f64 * (1.0 / self.resize_load_factor)).ceil();
        let new_size = new_size as usize;

        let mut new_map = Self::with_capacity(new_size.max(1));

        // Drain self into the new map.
        for mut v in self.buckets.drain(..) {
            for (k, v) in v.drain(..) {
                new_map.insert(k, v);
            }
        }

        // println!("got resize event to {new_size}");

        // Replace the map.
        *self = new_map;
    }

    pub fn load_factor(&self) -> f64 {
        self.entries as f64 / self.buckets.len() as f64
    }

    pub fn debug_info(&self) {
        let load = self.load_factor();
        println!(" load: {load}");
        println!(" buckets: {}", self.buckets.len());
        println!(" entries: {}", self.entries);
        for (i, b) in self.buckets.iter().enumerate() {
            println!(" b[{i}]: {}", b.len());
        }
    }

    pub fn load_factor_max(&self) -> f64 {
        self.load_factor_max
    }
    pub fn resize_load_factor(&self) -> f64 {
        self.resize_load_factor
    }
    pub fn set_load_factor_max(&mut self, v: f64) {
        self.load_factor_max = v;
    }
    pub fn set_resize_load_factor(&mut self, v: f64) {
        self.resize_load_factor = v;
    }
}

// Use this block to hold the 'std' methods.
impl<K: BucketKeyReq, V> BucketHashmap<K, V> {
    /// Create a new hashmap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a hashmap with at least this capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let bucket_count = (capacity as f64 / BUCKET_LOAD_FACTOR_MAX).ceil() as usize;
        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(Default::default());
        }
        Self {
            buckets,
            ..Default::default()
        }
    }

    /// Reserves at least this additional size.
    pub fn reserve(&mut self, additional: usize) {
        self.resize_to(self.entries + additional);
    }

    /// Shrinks to at least the specified capacity.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        let at_least = self.entries.max(min_capacity);
        self.resize_to(at_least);
    }

    /// Shrinks to minum value that holds the current size
    pub fn shrink_to_fit(&mut self) {
        self.resize_to(self.entries);
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

    /// Get a value by reference.
    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.calculate_bucket_index(&key);
        if let Some(index_in_bucket) = self.buckets[bucket_index]
            .iter()
            .position(|(bk, _)| *bk == *key)
        {
            self.buckets[bucket_index]
                .get(index_in_bucket)
                .map(|(_, v)| v)
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
        assert!(h.remove(&300).is_none());
        assert_eq!(h.len(), 33);
        assert!(!h.contains_key(&300));

        let z = h.clone();
        assert!(!z.contains_key(&300));
        assert_eq!(z.len(), 33);

        struct NonClone {}
        let non_clonable = BucketHashmap::<u64, NonClone>::new();
        // let z = non_clonable.clone();
        let _ = non_clonable;
    }

    #[test]
    fn test_fuzz() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut h = BucketHashmap::<u64, u64>::new();
        let mut r = std::collections::HashMap::<u64, u64>::new();

        // Insert 10000 into both.
        for _ in 0..10000 {
            let k: u64 = rng.gen();
            let v: u64 = rng.gen();
            h.insert(k, v);
            r.insert(k, v);
            assert!(h.contains_key(&k));
            assert!(r.contains_key(&k));
            assert_eq!(r.len(), h.len());
        }

        for _ in 0..10000 {
            let to_add: bool = rng.gen();
            if to_add {
                let k: u64 = rng.gen();
                let v: u64 = rng.gen();
                h.insert(k, v);
                r.insert(k, v);
                assert!(h.contains_key(&k));
                assert!(r.contains_key(&k));
                assert_eq!(r.len(), h.len());
            } else {
                // Find a value from the reference hashmap.
                let i = rng.gen_range(0..r.len());
                let k = *r.keys().skip(i).next().unwrap();
                let r_v = r.remove(&k);
                let h_v = h.remove(&k);
                assert_eq!(r_v, h_v);
                assert!(!h.contains_key(&k));
                assert!(!r.contains_key(&k));
                assert_eq!(r.len(), h.len());
            }
        }

        // Verify that the hashmaps contain equal things.
        for (k, v) in r.iter() {
            if let Some(hv) = h.get(k) {
                assert_eq!(*v, *hv);
            } else {
                assert!(false);
            }
        }
    }
}
