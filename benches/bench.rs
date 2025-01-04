#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, Criterion};
use hashmap_from_scratch::{BucketSeperateChainHashMap, CppStdUnorderedMapU64U64};
use rand::prelude::*;
use std::collections::HashMap;
use std::hint::black_box;

// Hashbrown bench is at https://github.com/rust-lang/hashbrown/blob/master/benches/bench.rs

macro_rules! default_benchmark {
    ($name:ident, $maptype:ident, $label:expr, $cycles:expr, $count:expr) => {
        fn $name(b: &mut Criterion) {
            let mut values = (0..$count).collect::<Vec<_>>();
            let mut rng = rand::thread_rng();
            values.shuffle(&mut rng);
            b.bench_function($label, |b| {
                b.iter(|| {
                    let mut h = $maptype::new();

                    for _ in 0..$cycles {
                        for (k, i) in values.iter().enumerate() {
                            h.insert(*i, *i);
                            assert_eq!(h.len(), k + 1);
                        }

                        assert_eq!(h.len(), $count);

                        for i in values.iter() {
                            assert!(h.contains_key(i));
                            assert!(h.get(i).is_some());
                            black_box(h.get(i));
                        }
                        for i in values.iter() {
                            h.remove(i);
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

type BucketHashmapU64U64 = BucketSeperateChainHashMap<u64, u64>;
default_benchmark!(
    criterion_bucket_separate_1k,
    BucketHashmapU64U64,
    "BucketSeparateCHain 1k",
    1000,
    1_000
);
default_benchmark!(
    criterion_bucket_separate_100k,
    BucketHashmapU64U64,
    "BucketSeparateCHain 100k",
    1,
    100_000
);

default_benchmark!(
    criterion_std_unordered_map_1k,
    CppStdUnorderedMapU64U64,
    "CppStdUnorderedMapU64U64 1k",
    1000,
    1_000
);

default_benchmark!(
    criterion_std_unordered_map_100k,
    CppStdUnorderedMapU64U64,
    "CppStdUnorderedMapU64U64 100k",
    1,
    100_000
);

criterion_group!(
    benches,
    criterion_std_unordered_map_1k,
    criterion_std_unordered_map_100k,
    criterion_std_1k,
    criterion_std_100k,
    criterion_bucket_separate_1k,
    criterion_bucket_separate_100k,
);

criterion_main!(benches);
