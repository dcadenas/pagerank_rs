use pagerank_rs::Pagerank;
use rand::{rngs::StdRng, Rng, SeedableRng};

// Calculates the pagerank for a graph of 1000000 nodes. The graph is
// generated randomly. Each node will have between 0 and 400 outgoing links.
// The first 3 nodes receive more incoming links than the rest
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 100_000;
    let mut page_rank = Pagerank::new(n);
    let mut rng = StdRng::seed_from_u64(5);

    for from in 0..n {
        for _ in 0..rng.gen_range(0..400) {
            let mut to = rng.gen_range(0..n);

            if to > 80_000 {
                to = rng.gen_range(0..3);
            }

            page_rank.link(from, to)?;
        }
    }

    let mut result = vec![0.0; n as usize];
    page_rank.rank(0.85, 0.001, |key, val| {
        result[key as usize] = val;
    });
    Ok(())
}
