#![forbid(unsafe_code)]

use std::{error::Error, fmt};

#[cfg(feature = "bellman_ford")]
pub mod bellman_ford;
pub use bellman_ford::*;

#[cfg(feature = "dijkstra")]
pub mod dijkstra;
pub use dijkstra::*;

/// Error type for graph algorithms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// Graph contains a negative weight cycle.
    NegativeWeightCycle,
}

impl Error for GraphError {}

impl fmt::Display for GraphError {
    /// Display the error message.
    ///
    /// # Arguments
    ///
    /// - `f`: Formatter.
    ///
    /// # Returns
    ///
    /// Result containing the formatted error message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A trait for graph search algorithms.
pub trait GraphAlgorithm {
    /// Type of node.
    type Node;

    /// Type of weight.
    type Weight;

    /// Run the graph algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: The starting node.
    ///
    /// # Returns
    ///
    /// Result containing a vector of shortest paths, or an error if applicable.
    fn run(&self, start: Self::Node) -> Result<Vec<Self::Weight>, GraphError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_error() {
        assert_eq!(
            format!("{}", GraphError::NegativeWeightCycle),
            "NegativeWeightCycle"
        );
    }
}
