//! This module provides the Pagerank struct which implements the PageRank algorithm
//! to measure the importance of nodes in a graph based on the incoming links from other nodes.
//! The algorithm assigns a numerical weighting to each element of a linked set of objects,
//! with the purpose of "measuring" its relative importance within the set.
//!
//! This implementation supports adding directed links between nodes, computing PageRank scores,
//! and managing the underlying graph data. It uses a simple iterative approach to converge to the
//! steady-state distribution of the PageRank values. The implementation leverages parallel computation
//! to improve performance on multi-core systems.
use crate::errors::PagerankError;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// A structure for managing and computing PageRank scores for nodes in a graph.
///
/// The Pagerank struct supports adding nodes and directed edges, and provides
/// a method to compute the PageRank scores for all nodes using the PageRank algorithm.
/// It internally maintains mappings between node identifiers and their indices in vectors
/// that store the graph's adjacency information.
///
/// Fields:
/// - in_links: A vector of vectors where each sub-vector contains the indices of nodes
///   that have an outgoing link to the node at the corresponding index.
/// - number_out_links: A vector where each element is the number of outgoing links
///   from the node at the corresponding index.
/// - current_available_index: The next available index for assigning to a new node.
/// - key_to_index: A mapping from node identifiers to their indices in the graph vectors.
/// - index_to_key: A mapping from indices in the graph vectors to node identifiers.
/// - capacity: The maximum number of nodes the Pagerank instance can handle.// and managing the underlying graph data.
pub struct Pagerank {
    in_links: Vec<Vec<usize>>,
    number_out_links: Vec<usize>,
    current_available_index: usize,
    key_to_index: HashMap<usize, usize>,
    index_to_key: HashMap<usize, usize>,
    capacity: usize,
}

impl Display for Pagerank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pagerank Struct:\n\
             InLinks: {:?}\n\
             NumberOutLinks: {:?}\n\
             CurrentAvailableIndex: {}\n\
             KeyToIndex: {:?}\n\
             IndexToKey: {:?}\n\
             Capacity: {}",
            self.in_links,
            self.number_out_links,
            self.current_available_index,
            self.key_to_index,
            self.index_to_key,
            self.capacity
        )
    }
}

impl Pagerank {
    /// Constructs a new Pagerank instance with the specified capacity.
    ///
    /// The capacity determines the maximum number of nodes the Pagerank instance can handle.
    /// It pre-allocates data structures to accommodate the graph up to this size.
    ///
    /// # Arguments
    ///
    /// * capacity - The maximum number of nodes in the graph.
    ///
    /// # Examples
    ///
    /// let pagerank = Pagerank::new(100); // Create a new Pagerank instance for a graph with up to 100 nodes.
    ///
    pub fn new(capacity: usize) -> Pagerank {
        Pagerank {
            in_links: vec![Vec::with_capacity(capacity); capacity],
            number_out_links: vec![0; capacity],
            current_available_index: 0,
            key_to_index: HashMap::with_capacity(capacity),
            index_to_key: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    fn key_as_array_index(&mut self, key: usize) -> Result<usize, PagerankError> {
        if self.current_available_index > self.capacity {
            let message = format!(
                "Exceeded the capacity of nodes, current available index: {}, capacity: {}",
                self.current_available_index, self.capacity,
            );
            return Err(PagerankError::CapacityError(message));
        }
        let index = self.key_to_index.entry(key).or_insert_with(|| {
            let new_index = self.current_available_index;
            self.index_to_key.insert(new_index, key);
            self.current_available_index += 1;
            new_index
        });
        Ok(*index)
    }

    fn update_in_links(&mut self, from_as_index: usize, to_as_index: usize) {
        self.in_links[to_as_index].push(from_as_index);
    }

    fn update_number_out_links(&mut self, from_as_index: usize) {
        self.number_out_links[from_as_index] += 1;
    }

    fn link_with_indices(&mut self, from_as_index: usize, to_as_index: usize) {
        self.update_in_links(from_as_index, to_as_index);
        self.update_number_out_links(from_as_index);
    }

    /// Adds a directed link from the from node to the to node.
    ///
    /// If the nodes do not exist, they will be created up to the capacity of the graph.
    ///
    /// # Arguments
    ///
    /// * from - The index of the node where the link originates.
    /// * to - The index of the node where the link points to.
    ///
    /// # Errors
    ///
    /// Returns a PagerankError if adding the link would exceed the graph's capacity.
    ///
    /// # Examples
    ///
    ///
    /// let mut pagerank = Pagerank::new(100);
    /// pagerank.link(1, 2).unwrap();
    ///
    pub fn link(&mut self, from: usize, to: usize) -> Result<(), PagerankError> {
        let from_as_index = self.key_as_array_index(from)?;
        let to_as_index = self.key_as_array_index(to)?;

        self.link_with_indices(from_as_index, to_as_index);
        Ok(())
    }

    fn calculate_dangling_nodes(&self) -> Vec<usize> {
        self.number_out_links
            .iter()
            .take(self.current_available_index)
            .enumerate()
            .filter(|&(_index, &out_links_count)| out_links_count == 0)
            .map(|(index, _)| index)
            .collect()
    }

    fn step(
        &self,
        following_prob: f64,
        t_over_size: f64,
        p: &[f64],
        dangling_nodes: &[usize],
        new_p: &mut [f64],
    ) {
        let size = p.len();
        let inner_product: f64 = dangling_nodes.par_iter().map(|&node| p[node]).sum();
        let inner_product_over_size = inner_product / size as f64;

        new_p.par_iter_mut().enumerate().for_each(|(i, new_p_i)| {
            let rank_sum: f64 = self.in_links[i]
                .par_iter()
                .map(|&index| p[index] / self.number_out_links[index] as f64)
                .sum();

            *new_p_i = following_prob * (rank_sum + inner_product_over_size) + t_over_size;
        });

        let v_sum: f64 = new_p.par_iter().sum();
        new_p.par_iter_mut().for_each(|x| *x /= v_sum);
    }

    fn calculate_change(p: &[f64], new_p: &[f64]) -> f64 {
        p.iter()
            .zip(new_p)
            .map(|(&old, &new)| (old - new).abs())
            .sum()
    }

    /// Computes the PageRank scores for all nodes in the graph.
    ///
    /// The computation iterates until the change in scores between iterations is below
    /// the specified tolerance, or until convergence is reached.
    ///
    /// # Arguments
    ///
    /// * following_prob - The probability of following a link (damping factor).
    /// * tolerance - The convergence tolerance; computation stops when the change in scores falls below this threshold.
    /// * result_func - A closure that is called with the node index and its PageRank score after convergence.
    ///
    /// # Examples
    ///
    ///
    /// let mut pagerank = Pagerank::new(100);
    /// // ... add links ...
    /// pagerank.rank(0.85, 1e-6, |node, score| {
    ///     println!("Node {}: {}", node, score);
    /// });
    ///
    pub fn rank(
        &mut self,
        following_prob: f64,
        tolerance: f64,
        mut result_func: impl FnMut(usize, f64),
    ) {
        let size = self.key_to_index.len();
        let inverse_of_size = 1.0 / size as f64;
        let t_over_size = (1.0 - following_prob) * inverse_of_size;
        let dangling_nodes = self.calculate_dangling_nodes();

        let mut p = vec![inverse_of_size; size]; // Current probabilities
        let mut new_p = vec![0.0; size]; // Buffer for new probabilities
        let mut change = 2.0;

        while change > tolerance {
            // Pass a mutable reference to new_p so that step can modify it directly
            self.step(following_prob, t_over_size, &p, &dangling_nodes, &mut new_p);
            change = Self::calculate_change(&p, &new_p);

            // Swap p and new_p for the next iteration
            std::mem::swap(&mut p, &mut new_p);
        }

        p.into_iter().enumerate().for_each(|(i, p_i)| {
            if let Some(&key) = self.index_to_key.get(&i) {
                result_func(key, p_i);
            }
        });
    }

    pub fn clear(&mut self) {
        self.in_links.iter_mut().for_each(|x| x.clear());
        self.number_out_links.fill(0);
        self.current_available_index = 0;
        self.key_to_index.clear();
        self.index_to_key.clear();
    }
}
