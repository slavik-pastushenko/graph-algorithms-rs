#![forbid(unsafe_code)]

// A* Algorithm
// Dijkstra's Algorithm
// Breadth-First Search (BFS)
// Depth-First Search (DFS)
// Bellman-Ford Algorithm
// Floyd-Warshall Algorithm

mod dijkstra;

/// A trait for graph search algorithms.
pub trait Algorithm {
    /// Run the algorithm.
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
