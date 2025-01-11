# Hashmap from scratch

Making a hashmap from scratch, just because I've never built one.

All keys are required to have the `Hash` and `Eq` trait. The overall structure is a typical hash map:
- The hash table has `N` buckets.
- For a key, the hash is calculated, the bucket / slot is determined by `hash(key) % N`.
- In the bucket, a linear search is performed to find the matching key.

For resizing / rehashing:
- The hash map tracks how much entries it contains, the load factor is the number of entries divided by `N`. If this would exceed `1.0`, the hashmap resizes to make the load factor `0.5`, so doubling the hashmap in size. When this happens a new hashmap is created of the appropriate size, and the old one is drained into the new one, re-calculating into which bucket each key would go.


The generic implementation supports any bucket type that implements the `BucketInterface` trait, this allows using buckets of type `Vec<(K, V)>` or `SmallVec<(K, V), M>`. This has the nice property that we can put the actual bucket inside of the main buckets container, which means that if no hash collisions occur, everything is inside of the main container.

The `bucket_seperate_chain_simple.rs` file contains the non-generic version.

Misc notes:
- Branch `compare-cpp` has a comparison with a c++ unordered_map, but the comparison isn't really fair as the hasher for `u64` in c++ is a unity hash function, so it just becomes an indexed vector and no hash collisions will ever happen with the current benchmark.

## License
License is `BSD-3-Clause`.
