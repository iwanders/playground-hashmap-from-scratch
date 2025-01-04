# Hashmap from scratch

Making a hashmap from scratch, just because I've never built one.

- Branch `in-bucket-head` hold an implementation that uses a tinyvec for each bucket with a single value in the main bucket list. This makes data nice and compact if no hash collisions occur. Small benchmark is close to the `std::collections::HashMap` in performance, large still significantly slower.
- Branch `compare-cpp` has a comparison with a c++ unordered_map, but the comparison isn't really fair as the hasher for `u64` in c++ is a unity hash function, so it just becomes an indexed vector and no hash collisions will ever happen with the current benchmark.

## License
License is `BSD-3-Clause`.
