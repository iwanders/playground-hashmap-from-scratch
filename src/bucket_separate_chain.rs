use std::hash::{Hash, Hasher};

pub trait BucketKeyReq: Hash + Eq {}
impl<T: Hash + Eq> BucketKeyReq for T {}

const DEFAULT_BUCKET_LOAD_FACTOR_MAX: f64 = 1.0;
const DEFAULT_BUCKET_RESIZE_LOAD_FACTOR: f64 = 0.5;

pub trait BucketInterface<K, V>: Sized {
    fn len(&self) -> usize;

    fn drain_into_map<M: HashMapInsertTrait<K, V>>(&mut self, map: &mut M);
    fn vec_iter_mut<'a>(&'a mut self) -> impl std::iter::Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a;
    fn vec_iter<'a>(&'a self) -> impl std::iter::Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a;
    fn vec_swap_remove(&mut self, position: usize) -> (K, V);
    fn vec_get(&self, index: usize) -> Option<&(K, V)>;
    fn vec_push(&mut self, value: (K, V));
}

pub trait BucketContainerReq<K, V>:
    std::ops::Index<usize> + BucketInterface<K, V> + Default
{
}
impl<K, V, T: std::ops::Index<usize> + BucketInterface<K, V> + Default> BucketContainerReq<K, V>
    for T
{
}

impl<K: BucketKeyReq, V> BucketInterface<K, V> for Vec<(K, V)> {
    fn len(&self) -> usize {
        self.len()
    }
    fn drain_into_map<M: HashMapInsertTrait<K, V>>(&mut self, map: &mut M) {
        for (k, v) in self.drain(..) {
            map.map_insert(k, v);
        }
    }
    fn vec_iter_mut<'a>(&'a mut self) -> impl std::iter::Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.iter_mut()
    }
    fn vec_iter<'a>(&'a self) -> impl std::iter::Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.iter()
    }
    fn vec_swap_remove(&mut self, position: usize) -> (K, V) {
        self.swap_remove(position)
    }
    fn vec_get(&self, index: usize) -> Option<&(K, V)> {
        self.get(index)
    }
    fn vec_push(&mut self, value: (K, V)) {
        self.push(value);
    }
}

pub trait HashMapInsertTrait<K, V> {
    fn map_insert(&mut self, k: K, v: V);
}

impl<K: BucketKeyReq, V, BucketType: BucketContainerReq<K, V>> HashMapInsertTrait<K, V>
    for BucketSeperateChainHashMap<K, V, BucketType>
{
    fn map_insert(&mut self, k: K, v: V) {
        self.insert(k, v);
    }
}

pub type HashmapChainVec<K, V> = BucketSeperateChainHashMap<K, V, Vec<(K, V)>>;

#[derive(Debug)]
pub struct BucketSeperateChainHashMap<K: BucketKeyReq, V, BucketType: BucketContainerReq<K, V>> {
    entries: usize,
    load_factor_max: f64,
    resize_load_factor: f64,
    buckets: Vec<BucketType>,
    _z: std::marker::PhantomData<(K, V)>,
}
impl<K: BucketKeyReq, V, BucketType: BucketContainerReq<K, V>> Default
    for BucketSeperateChainHashMap<K, V, BucketType>
{
    fn default() -> Self {
        Self {
            entries: 0,
            load_factor_max: DEFAULT_BUCKET_LOAD_FACTOR_MAX,
            resize_load_factor: DEFAULT_BUCKET_RESIZE_LOAD_FACTOR,
            buckets: vec![BucketType::default()],
            _z: Default::default(),
        }
    }
}
impl<K: BucketKeyReq + Clone, V: Clone, BucketType: BucketContainerReq<K, V> + Clone> Clone
    for BucketSeperateChainHashMap<K, V, BucketType>
{
    fn clone(&self) -> Self {
        Self {
            entries: self.entries,
            load_factor_max: self.load_factor_max,
            resize_load_factor: self.resize_load_factor,
            buckets: self.buckets.clone(),
            _z: Default::default(),
        }
    }
}

impl<K: BucketKeyReq, V, BucketType: BucketContainerReq<K, V>>
    BucketSeperateChainHashMap<K, V, BucketType>
{
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
        new_map.resize_load_factor = self.resize_load_factor;
        new_map.load_factor_max = self.load_factor_max;

        // Drain self into the new map.
        for mut v in self.buckets.drain(..) {
            v.drain_into_map(&mut new_map)
        }

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
        // for (i, b) in self.buckets.iter().enumerate() {
        // println!(" b[{i}]: {}", b.len());
        // }
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
impl<K: BucketKeyReq, V, BucketType: BucketContainerReq<K, V>>
    BucketSeperateChainHashMap<K, V, BucketType>
{
    /// Create a new hashmap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a hashmap with at least this capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let bucket_count = (capacity as f64 / DEFAULT_BUCKET_LOAD_FACTOR_MAX).ceil() as usize;
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
        for (bk, bv) in b.vec_iter_mut() {
            if *bk == key {
                *bv = value;
                return;
            }
        }

        let b: &mut _ = &mut self.buckets[bucket_index];
        // We did not find this key already, so we append it to the bucket.
        self.entries += 1;
        b.vec_push((key, value));

        // Resize if that was actually necessary
        self.resize_to(self.entries);
    }

    /// Check if a key exists.
    pub fn contains_key(&self, k: &K) -> bool {
        let bucket_index = self.calculate_bucket_index(k);
        // We found the bucket.
        let b: &_ = &self.buckets[bucket_index];
        // Search in that bucket.
        for (bk, _bv) in b.vec_iter() {
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

    /// Return if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.entries == 0
    }

    /// Remove an entry from the hashmap.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.calculate_bucket_index(&key);

        // Why does our implementation need an intermediate, but the 'real' one doesn't?
        let intermediate = self.buckets[bucket_index]
            .vec_iter()
            .position(|(bk, _)| *bk == *key);

        if let Some(index_in_bucket) = intermediate {
            let v = self.buckets[bucket_index].vec_swap_remove(index_in_bucket);
            self.entries -= 1;
            return Some(v.1);
        } else {
            return None;
        };
        /*
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
        */
    }

    /// Get a value by reference.
    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.calculate_bucket_index(&key);

        let intermediate = self.buckets[bucket_index]
            .vec_iter()
            .position(|(bk, _)| *bk == *key);
        if let Some(index_in_bucket) = intermediate {
            self.buckets[bucket_index]
                .vec_get(index_in_bucket)
                .map(|(_, v)| v)
        } else {
            None
        }
        /*
        if let Some(index_in_bucket) = self.buckets[bucket_index]
            .iter()
            .position(|(bk, _)| *bk == *key)
        {
            self.buckets[bucket_index]
                .get(index_in_bucket)
                .map(|(_, v)| v)
        } else {
            None
        }*/
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bucket_seperate_chain() {
        let mut h = HashmapChainVec::<u64, u64>::new();
        h.insert(300, 3);
        h.insert(500, 8);
        assert!(h.contains_key(&300));
        assert!(h.contains_key(&500));
        assert!(!h.contains_key(&8));
        for i in 0..32 {
            h.insert(i, i);
            // println!("h size: {}", h.len());
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
    }

    #[test]
    fn test_bucket_seperate_chain_nonclone() {
        struct NonClone {}
        let non_clonable = HashmapChainVec::<u64, NonClone>::new();
        // let z = non_clonable.clone();
        let _ = non_clonable;
    }

    #[test]
    fn test_fuzz() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut h = HashmapChainVec::<u64, u64>::new();
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
