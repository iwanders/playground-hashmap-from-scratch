use criterion::{criterion_group, criterion_main, Criterion};
use hashmap_from_scratch::BucketSeperateChainHashMap;
use std::collections::HashMap;
use std::hint::black_box;

// Hashbrown bench is at https://github.com/rust-lang/hashbrown/blob/master/benches/bench.rs

macro_rules! default_benchmark {
    ($name:ident, $maptype:ident, $label:expr) => {
        fn $name(b: &mut Criterion) {
            b.bench_function($label, |b| {
                b.iter(|| {
                    let mut h = $maptype::new();

                    // Each loop triggers one rehash
                    for _ in 0..1000 {
                        for i in 0..1000 {
                            h.insert(i, i);
                        }

                        assert_eq!(h.len(), 1000);

                        for i in 0..1000 {
                            assert!(h.contains_key(&i));
                            assert!(h.get(&i).is_some());
                        }
                        for i in 0..1000 {
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
default_benchmark!(criterion_std, StdHashmapU64U64, "std HashMap");

type BucketHashmapU64U64 = BucketSeperateChainHashMap<u64, u64>;
default_benchmark!(
    criterion_bucket_separate,
    BucketHashmapU64U64,
    "BucketSeparateCHain"
);

criterion_group!(benches, criterion_std, criterion_bucket_separate);

criterion_main!(benches);
