# Hashmap from scratch

Making a hashmap from scratch, just because I've never built one.

- The `bucket_separate_chain.rs` contains a bucket-based hashmap, generalized over the bucket container, which can then either be a `Vec<(K, V)>` or `SmallVec<(K, V)>`, this made code a bit messy, so the `bucket_seperate_chain_simple.rs` file contains the non-generic version.
- Branch `compare-cpp` has a comparison with a c++ unordered_map, but the comparison isn't really fair as the hasher for `u64` in c++ is a unity hash function, so it just becomes an indexed vector and no hash collisions will ever happen with the current benchmark.

## License
License is `BSD-3-Clause`.
