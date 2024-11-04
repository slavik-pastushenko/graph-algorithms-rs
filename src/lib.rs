#![forbid(unsafe_code)]

use std::{error::Error, fmt};

#[cfg(feature = "bellman_ford")]
pub mod bellman_ford;
pub use bellman_ford::*;

#[cfg(feature = "dijkstra")]
pub mod dijkstra;
pub use dijkstra::*;

#[cfg(feature = "floyd_warshall")]
pub mod floyd_warshall;
pub use floyd_warshall::*;

/// Error type for graph algorithms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// Graph contains a negative weight cycle.
    NegativeWeightCycle,

    /// Graph does not contain a start node.
    MissingStartNode,
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
    /// - `start`: Starting node, if applicable.
    ///
    /// # Returns
    ///
    /// Result containing the weight of the shortest path, or an error.
    fn run(&self, start: Option<Self::Node>) -> Result<Self::Weight, GraphError>;
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

        assert_eq!(
            format!("{}", GraphError::MissingStartNode),
            "MissingStartNode"
        );
    }
}
