use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pagerank_rs::Pagerank;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

fn pagerank_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pagerank_group");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(22));

    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    group.bench_function(BenchmarkId::new("pagerank", ""), |b| {
        let n = 100_000;
        let mut page_rank = Pagerank::new(n);

        b.iter(|| {
            for from in 0..n {
                for _ in 0..rng.gen_range(0..400) {
                    let mut to = rng.gen_range(0..n);

                    if to > 80_000 {
                        to = rng.gen_range(0..3);
                    }

                    page_rank.link(black_box(from), black_box(to)).unwrap();
                }
            }

            let mut result = vec![0.0; n as usize];
            page_rank.rank(black_box(0.85), black_box(0.001), |key, val| {
                result[key as usize] = val;
            });

            page_rank.clear();
        });
    });

    group.finish();
}

criterion_group!(benches, pagerank_benchmark);
criterion_main!(benches);
