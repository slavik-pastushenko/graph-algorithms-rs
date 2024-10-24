use std::{cmp::Ordering, collections::BinaryHeap};

use crate::Algorithm;

/// Dijkstra's Algorithm.
/// Finds the shortest path from a starting node to all other nodes in a weighted graph.
/// Docs: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
#[derive(Debug)]
pub struct Dijkstra {
    /// Graph to search.
    graph: Vec<Vec<(usize, usize)>>,
}

/// State of the algorithm.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    /// Cost of the path.
    cost: usize,

    /// Position of the node.
    position: usize,
}

impl Ord for State {
    /// Compare two states.
    ///
    /// # Arguments
    ///
    /// - `other`: The other state to compare.
    ///
    /// # Returns
    ///
    /// Ordering of the two states.
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    /// Compare two states partially.
    ///
    /// # Arguments
    ///
    /// - `other`: The other state to compare.
    ///
    /// # Returns
    ///
    /// Ordering of the two states.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Dijkstra {
    /// Create a new instance of Dijkstra's Algorithm.
    ///
    /// # Returns
    ///
    /// A new instance of Dijkstra's Algorithm.
    #[cfg(test)]
    pub fn new() -> Self {
        Dijkstra { graph: vec![] }
    }

    /// Set the node of the graph.
    ///
    /// # Arguments
    ///
    /// - `node`: Node of the graph.
    #[cfg(test)]
    pub fn set_node(&mut self, node: Vec<(usize, usize)>) {
        self.graph.push(node);
    }

    /// Set the nodes of the graph.
    ///
    /// # Arguments
    ///
    /// - `nodes`: A vector of nodes of the graph.
    #[cfg(test)]
    pub fn set_nodes(&mut self, mut nodes: Vec<Vec<(usize, usize)>>) {
        self.graph.append(&mut nodes);
    }
}

impl Algorithm for Dijkstra {
    /// Run Dijkstra's Algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: The starting node.
    ///
    /// # Returns
    ///
    /// A vector of the shortest path from the starting node to all other nodes.
    fn run(&self, start: usize) -> Vec<usize> {
        let mut priority_queue = BinaryHeap::new();
        let mut distances = vec![usize::MAX; self.graph.len()];

        distances[start] = 0;
        priority_queue.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = priority_queue.pop() {
            // Determine if the current shortest path is already known.
            // If it is, skip the current node.
            if cost > distances[position] {
                continue;
            }

            for &(neighbor, weight) in &self.graph[position] {
                let next = State {
                    cost: cost + weight,
                    position: neighbor,
                };

                // Determine if the new path is shorter than the current shortest path.
                // If it is, update the shortest path.
                if next.cost < distances[neighbor] {
                    distances[neighbor] = next.cost;
                    priority_queue.push(next);
                }
            }
        }

        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node_graph() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![]]);

        assert_eq!(dijkstra.run(0), vec![0]);
    }

    #[test]
    fn test_two_node_graph() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_node(vec![(1, 1)]);
        dijkstra.set_node(vec![]);

        assert_eq!(dijkstra.run(0), vec![0, 1]);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, usize::MAX]);
    }

    #[test]
    fn test_simple_path() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1)], vec![(2, 1)], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, 1, 2]);
    }

    #[test]
    fn test_multiple_paths() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1), (2, 4)], vec![(2, 2)], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, 1, 3]);
    }

    #[test]
    fn test_graph_with_cycle() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1)], vec![(2, 1)], vec![(0, 1)]]);

        assert_eq!(dijkstra.run(0), vec![0, 1, 2]);
    }

    #[test]
    fn test_graph_with_multiple_shortest_paths() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1), (2, 1)], vec![(2, 1)], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, 1, 1]);
    }

    #[test]
    fn test_large_graph() {
        let mut dijkstra = Dijkstra::new();
        let nodes = vec![
            vec![(1, 1), (2, 4), (3, 7)],
            vec![(2, 2), (3, 5), (4, 12)],
            vec![(3, 1), (4, 3)],
            vec![(4, 2), (5, 8)],
            vec![(5, 1)],
            vec![],
        ];
        dijkstra.set_nodes(nodes);

        assert_eq!(dijkstra.run(0), vec![0, 1, 3, 4, 6, 7]);
    }

    #[test]
    fn test_negative_weights() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, usize::MAX)], vec![(2, usize::MAX)], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, usize::MAX, usize::MAX]);
    }

    #[test]
    fn test_large_weights() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1000)], vec![(2, 1000)], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, 1000, 2000]);
    }

    #[test]
    fn test_no_edges() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![], vec![], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, usize::MAX, usize::MAX]);
    }

    #[test]
    fn test_multiple_edges_to_same_node() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1), (1, 2)], vec![(2, 1)], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, 1, 2]);
    }

    #[test]
    fn test_graph_with_isolated_node() {
        let mut dijkstra = Dijkstra::new();
        dijkstra.set_nodes(vec![vec![(1, 1)], vec![(2, 1)], vec![], vec![]]);

        assert_eq!(dijkstra.run(0), vec![0, 1, 2, usize::MAX]);
    }
}
