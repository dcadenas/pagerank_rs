use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use graph::prelude::*;
use pagerank_rs::Pagerank;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use simple_pagerank::Pagerank as SimplePagerank;
use std::time::Duration;

fn pagerank_graph_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pagerank_graph_group");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(22));

    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    group.bench_function(BenchmarkId::new("pagerank_graph", ""), |b| {
        b.iter(|| {
            let n = 100_000;
            let mut edges = Vec::new();
            for from in 0..n {
                for _ in 0..rng.gen_range(0..400) {
                    let to = rng.gen_range(0..n);
                    edges.push((from, to));
                }
            }

            let graph: DirectedCsrGraph<usize> = GraphBuilder::new().edges(edges).build();

            let (_ranks, _, _) = page_rank(&graph, PageRankConfig::new(10, 1E-4, black_box(0.85)));
        });
    });

    group.finish();
}

fn simple_pagerank_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_pagerank_group");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(22));

    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    group.bench_function(BenchmarkId::new("simple_pagerank", ""), |b| {
        b.iter(|| {
            let n = 100_000;
            let mut pr = SimplePagerank::<usize>::new();

            for from in 0..n {
                for _ in 0..rng.gen_range(0..400) {
                    let to = rng.gen_range(0..n);
                    pr.add_edge(black_box(from), black_box(to));
                }
            }

            pr.nodes().iter().for_each(|(_node, _score)| {});
        });
    });

    group.finish();
}

fn pagerank_rs_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pagerank_rs_group");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(22));

    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    group.bench_function(BenchmarkId::new("pagerank_rs", ""), |b| {
        let n = 100_000;
        let mut pagerank = Pagerank::new(n);

        b.iter(|| {
            for from in 0..n {
                for _ in 0..rng.gen_range(0..400) {
                    let to = rng.gen_range(0..n);
                    pagerank.link(black_box(from), black_box(to)).unwrap();
                }
            }

            pagerank.rank(black_box(0.85), black_box(0.01));
            pagerank.clear();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    pagerank_graph_benchmark,
    simple_pagerank_benchmark,
    pagerank_rs_benchmark,
);
criterion_main!(benches);
