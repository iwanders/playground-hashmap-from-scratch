#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, Criterion};
use hashmap_from_scratch::{HashmapChainSmallVec, HashmapChainVec};
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
macro_rules! random_benchmark {
    ($name:ident, $maptype:ident, $label:expr, $cycles:expr, $count:expr) => {
        fn $name(b: &mut Criterion) {
            use rand::thread_rng;
            use rand_distr::{Distribution, Uniform};
            let mut rng = thread_rng();
            let normal = Uniform::new(0, u64::MAX);
            let mut values = vec![];
            for _ in 0..$count {
                let x = normal.sample(&mut rng);
                values.push(x);
            }
            b.bench_function($label, |b| {
                b.iter(|| {
                    let mut h = $maptype::new();

                    for _ in 0..$cycles {
                        for i in 0..$count {
                            let v = values[i];
                            h.insert(v, v);
                        }

                        assert_eq!(h.len(), $count);

                        for i in 0..$count {
                            let v = values[i];
                            assert!(h.contains_key(&v));
                            assert!(h.get(&v).is_some());
                            black_box(h.get(&v));
                        }
                        for i in 0..$count {
                            let v = values[i];
                            h.remove(&v);
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
random_benchmark!(
    criterion_std_1k_rng,
    StdHashmapU64U64,
    "StdHashmapU64U64 1k rng",
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

type BucketHashmapU64U64 = HashmapChainVec<u64, u64>;
default_benchmark!(
    criterion_bucket_separate_1k,
    BucketHashmapU64U64,
    "BucketSeparateCHain 1k",
    1000,
    1_000
);

random_benchmark!(
    criterion_bucket_separate_1k_rng,
    BucketHashmapU64U64,
    "BucketHashmapU64U64 1k rng",
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

type BucketHashmapU64U64SmallVec1 = HashmapChainSmallVec<u64, u64, 1>;
default_benchmark!(
    criterion_bucket_separate_smallvec1_1k,
    BucketHashmapU64U64SmallVec1,
    "BucketHashmapU64U64SmallVec1 1k",
    1000,
    1_000
);
random_benchmark!(
    criterion_bucket_separate_smallvec1_1k_rng,
    BucketHashmapU64U64SmallVec1,
    "BucketHashmapU64U64SmallVec1 1k rng",
    1000,
    1_000
);

default_benchmark!(
    criterion_bucket_separate_smallvec1_100k,
    BucketHashmapU64U64SmallVec1,
    "BucketHashmapU64U64SmallVec1 100k",
    1,
    100_000
);

type BucketHashmapU64U64SmallVec2 = HashmapChainSmallVec<u64, u64, 2>;
default_benchmark!(
    criterion_bucket_separate_smallvec2_1k,
    BucketHashmapU64U64SmallVec2,
    "BucketHashmapU64U64SmallVec2 1k",
    1000,
    1_000
);
random_benchmark!(
    criterion_bucket_separate_smallvec2_1k_rng,
    BucketHashmapU64U64SmallVec2,
    "BucketHashmapU64U64SmallVec2 1k rng",
    1000,
    1_000
);

default_benchmark!(
    criterion_bucket_separate_smallvec2_100k,
    BucketHashmapU64U64SmallVec2,
    "BucketHashmapU64U64SmallVec2 100k",
    1,
    100_000
);

criterion_group!(
    benches,
    criterion_std_1k,
    criterion_std_100k,
    criterion_bucket_separate_1k,
    criterion_bucket_separate_100k,
    criterion_bucket_separate_smallvec1_1k,
    criterion_bucket_separate_smallvec1_100k,
    criterion_bucket_separate_smallvec2_1k,
    criterion_bucket_separate_smallvec2_100k,
    criterion_std_1k_rng,
    criterion_bucket_separate_1k_rng,
    criterion_bucket_separate_smallvec2_1k_rng,
);

criterion_main!(benches);
