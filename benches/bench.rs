#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, Criterion};
use hashmap_from_scratch::{BucketInChainHashMap, BucketSeperateChainHashMap};
use std::collections::HashMap;
use std::hint::black_box;

// Hashbrown bench is at https://github.com/rust-lang/hashbrown/blob/master/benches/bench.rs

macro_rules! default_benchmark {
    ($name:ident, $maptype:ident, $label:expr, $cycles:expr, $count:expr) => {
        fn $name(b: &mut Criterion) {
            b.bench_function($label, |b| {
                b.iter(|| {
                    let mut h = $maptype::new();

                    for _ in 0..$cycles {
                        for i in 0..$count {
                            h.insert(i, i);
                        }

                        assert_eq!(h.len(), $count);

                        for i in 0..$count {
                            assert!(h.contains_key(&i));
                            assert!(h.get(&i).is_some());
                            black_box(h.get(&i));
                        }
                        for i in 0..$count {
                            h.remove(&i);
                        }
                        assert!(h.is_empty());
                    }
                })
            });
        }
    };
}

type StdHashmapU64U64 = HashMap<u64, u64>;
default_benchmark!(
    criterion_std_1k,
    StdHashmapU64U64,
    "std HashMap 1k",
    1000,
    1_000
);
default_benchmark!(
    criterion_std_100k,
    StdHashmapU64U64,
    "std HashMap 100k",
    1,
    100_000
);

type BucketHashmapSeparateU64U64 = BucketSeperateChainHashMap<u64, u64>;
default_benchmark!(
    criterion_bucket_separate_1k,
    BucketHashmapSeparateU64U64,
    "BucketSeparateCHain 1k",
    1000,
    1_000
);
default_benchmark!(
    criterion_bucket_separate_100k,
    BucketHashmapSeparateU64U64,
    "BucketSeparateCHain 100k",
    1,
    100_000
);

type BucketHashmapU64U64 = BucketInChainHashMap<u64, u64>;
default_benchmark!(
    criterion_bucket_1k,
    BucketHashmapU64U64,
    "BucketCHain 1k",
    1000,
    1_000
);
default_benchmark!(
    criterion_bucket_100k,
    BucketHashmapU64U64,
    "BucketCHain 100k",
    1,
    100_000
);

criterion_group!(
    benches,
    criterion_std_1k,
    criterion_std_100k,
    criterion_bucket_separate_1k,
    criterion_bucket_separate_100k,
    criterion_bucket_1k,
    criterion_bucket_100k,
);

criterion_main!(benches);
