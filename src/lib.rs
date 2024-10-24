#![forbid(unsafe_code)]

pub mod dijkstra;

pub use dijkstra::*;

/// A trait for graph search algorithms.
pub trait GraphAlgorithm {
    /// Run the graph algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: The starting node.
    ///
    /// # Returns
    ///
    /// A vector of the shortest path from the starting node to all other nodes.
    fn run(&self, start: usize) -> Vec<usize>;
}
