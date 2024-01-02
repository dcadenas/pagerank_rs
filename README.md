# pagerank_rs
[![Coverage Status](https://coveralls.io/repos/github/dcadenas/pagerank_rs/badge.svg?branch=master)](https://coveralls.io/github/dcadenas/pagerank_rs?branch=master)

`pagerank_rs` is a Rust library for computing PageRank, designed to accommodate graphs of varying sizes. It is intended for network analysis applications, such as social networks or academic citations, and offers a practical approach to determine the significance of nodes within a network.

![PageRank Example](http://upload.wikimedia.org/wikipedia/commons/thumb/f/fb/PageRanks-Example.svg/596px-PageRanks-Example.svg.png)

For an in-depth look at the PageRank algorithm, visit its [Wikipedia page](http://en.wikipedia.org/wiki/PageRank).

## Installation

Add `pagerank_rs` to your Cargo.toml:

```toml
[dependencies]
pagerank_rs = "0.1.0"
```

## Usage

Here's a simple example of how to use pagerank_rs:

```rust
use pagerank_rs::Pagerank;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = Pagerank::new(3);
    graph.link(0, 1)?;
    graph.link(1, 2)?;

    let probability_of_following_a_link = 0.85;
    let tolerance = 0.0001;

    let result = graph.rank(probability_of_following_a_link, tolerance);
    for (node_id, rank) in result {
        println!("Node {} rank is {}", node_id, rank);
    }

    Ok(())
}
```


This code will output the PageRank of each node in the graph. Node IDs can be any usize integer. The rank values are returned through a closure to avoid allocating a large result object.

For more complex examples and usage patterns, please refer to the unit tests in the repository.

## Contributing

We welcome contributions to pagerank_rs! Here's how to get started:

- Fork the project on GitHub.
- Create a new branch for your feature or bug fix.
- Write code and add tests for your changes.
- Ensure that all tests pass.
- Submit a pull request against the main branch.

Please follow the Rust coding conventions and include appropriate documentation.

## License

`pagerank_rs` is provided under the [MIT License](https://github.com/dcadenas/pagerank_rs/blob/master/LICENSE). See the LICENSE file for more details.

## Author

[Daniel Cadenas](https://github.com/dcadenas)