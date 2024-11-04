use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::{GraphAlgorithm, GraphError};

/// Dijkstra's Algorithm.
/// Find the shortest path from a starting node to all other nodes in a weighted graph.
#[derive(Debug, Clone)]
pub struct DijkstraAlgorithm {
    /// Graph to search.
    pub graph: HashMap<usize, Vec<(usize, usize)>>,
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

impl Default for DijkstraAlgorithm {
    /// Create a new default instance of Dijkstra's Algorithm.
    ///
    /// # Returns
    ///
    /// New default instance of Dijkstra's Algorithm.
    fn default() -> Self {
        Self::new()
    }
}

impl DijkstraAlgorithm {
    /// Create a new instance of Dijkstra's Algorithm.
    ///
    /// # Returns
    ///
    /// New instance of Dijkstra's Algorithm.
    pub fn new() -> Self {
        DijkstraAlgorithm {
            graph: HashMap::new(),
        }
    }

    /// Set the node of the graph.
    ///
    /// # Arguments
    ///
    /// - `node`: Node of the graph.
    /// - `edges`: Edges of the node.
    pub fn set_node(&mut self, node: usize, edges: Vec<(usize, usize)>) {
        self.graph.insert(node, edges);
    }

    /// Set the nodes of the graph.
    ///
    /// # Arguments
    ///
    /// - `nodes`: Vector of nodes and their edges.
    pub fn set_nodes(&mut self, nodes: Vec<(usize, Vec<(usize, usize)>)>) {
        for (node, edges) in nodes {
            self.graph.insert(node, edges);
        }
    }
}

impl GraphAlgorithm for DijkstraAlgorithm {
    /// Type of node.
    type Node = usize;

    /// Type of weight.
    type Weight = Vec<usize>;

    /// Run Dijkstra's Algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: Starting node.
    ///
    /// # Returns
    ///
    /// Vector of the shortest path from the starting node to all other nodes.
    fn run(&self, start: Option<Self::Node>) -> Result<Self::Weight, GraphError> {
        let start = start.ok_or(GraphError::MissingStartNode)?;

        let mut priority_queue = BinaryHeap::new();
        let mut distances = HashMap::new();
        let mut result = vec![usize::MAX; self.graph.len()];

        distances.insert(start, 0);
        priority_queue.push(State {
            cost: 0,
            position: start,
        });

        while let Some(state) = priority_queue.pop() {
            // Determine if the current shortest path is already known.
            // If it is, skip the current node.
            if distances
                .get(&state.position)
                .map(|&d| state.cost > d)
                .unwrap_or(false)
            {
                continue;
            }

            if let Some(neighbors) = self.graph.get(&state.position) {
                for &(neighbor, weight) in neighbors {
                    let next = State {
                        cost: state.cost + weight,
                        position: neighbor,
                    };

                    // Determine if the new path is shorter than the current shortest path.
                    // If it is, update the shortest path.
                    if distances
                        .get(&neighbor)
                        .map(|&d| next.cost < d)
                        .unwrap_or(true)
                    {
                        distances.insert(neighbor, next.cost);
                        priority_queue.push(next);
                    }
                }
            }
        }

        // Convert the distances to a vector of shortest paths.
        for (node, dist) in distances.into_iter() {
            if node < result.len() {
                result[node] = dist;
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let algorithm = DijkstraAlgorithm::new();
        let algorithm_default = DijkstraAlgorithm::default();

        assert_eq!(algorithm.graph.len(), 0);
        assert_eq!(algorithm_default.graph.len(), 0);
    }

    #[test]
    fn test_missing_start_node() {
        let algorithm = DijkstraAlgorithm::new();

        assert_eq!(algorithm.run(None), Err(GraphError::MissingStartNode));
    }

    #[test]
    fn test_run() {
        let mut algorithm = DijkstraAlgorithm::new();
        let nodes = vec![
            (0, vec![(1, 1), (2, 4), (3, 7)]),
            (1, vec![(2, 2), (3, 5), (4, 12)]),
            (2, vec![(3, 1), (4, 3)]),
            (3, vec![(4, 2), (5, 8)]),
            (4, vec![(5, 1), (6, 5)]),
            (5, vec![(6, 2), (7, 3)]),
            (6, vec![(7, 1), (8, 4)]),
            (7, vec![(8, 2), (9, 6)]),
            (8, vec![(9, 1)]),
            (9, vec![(10, 2), (11, 3)]),
            (10, vec![(11, 1), (12, 4)]),
            (11, vec![(12, 2), (13, 6)]),
            (12, vec![(13, 1), (14, 5)]),
            (13, vec![(14, 2), (15, 3)]),
            (14, vec![(15, 1), (16, 4)]),
            (15, vec![(16, 2), (17, 6)]),
            (16, vec![(17, 1), (18, 5)]),
            (17, vec![(18, 2), (19, 3)]),
            (18, vec![(19, 1)]),
            (19, vec![]),
        ];
        algorithm.set_nodes(nodes);

        assert_eq!(
            algorithm.run(Some(0)).unwrap(),
            vec![0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25, 27, 28]
        );
    }

    #[test]
    fn test_run_single_node_graph() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![(0, vec![])]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0]);
    }

    #[test]
    fn test_run_two_node_graph() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_node(0, vec![(1, 1)]);
        algorithm.set_node(1, vec![]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1]);
    }

    #[test]
    fn test_run_disconnected_graph() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![(0, vec![]), (1, vec![])]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, usize::MAX]);
    }

    #[test]
    fn test_run_simple_path() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![(0, vec![(1, 1)]), (1, vec![(2, 1)]), (2, vec![])]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn test_run_multiple_paths() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![
            (0, vec![(1, 1), (2, 4)]),
            (1, vec![(2, 2)]),
            (2, vec![]),
        ]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1, 3]);
    }

    #[test]
    fn test_run_graph_with_cycle() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![
            (0, vec![(1, 1)]),
            (1, vec![(2, 1)]),
            (2, vec![(0, 1)]),
        ]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn test_run_graph_with_multiple_shortest_paths() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![
            (0, vec![(1, 1), (2, 1)]),
            (1, vec![(2, 1)]),
            (2, vec![]),
        ]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1, 1]);
    }

    #[test]
    fn test_run_large_weights() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![
            (0, vec![(1, 1000)]),
            (1, vec![(2, 1000)]),
            (2, vec![]),
        ]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1000, 2000]);
    }

    #[test]
    fn test_run_no_edges() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![(0, vec![]), (1, vec![]), (2, vec![])]);

        assert_eq!(
            algorithm.run(Some(0)).unwrap(),
            vec![0, usize::MAX, usize::MAX]
        );
    }

    #[test]
    fn test_run_multiple_edges_to_same_node() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![
            (0, vec![(1, 1), (1, 2)]),
            (1, vec![(2, 1)]),
            (2, vec![]),
        ]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn test_run_graph_with_isolated_node() {
        let mut algorithm = DijkstraAlgorithm::new();
        algorithm.set_nodes(vec![
            (0, vec![(1, 1)]),
            (1, vec![(2, 1)]),
            (2, vec![]),
            (3, vec![]),
        ]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 1, 2, usize::MAX]);
    }
}
