use crate::{GraphAlgorithm, GraphError};

/// Edge in the graph.
#[derive(Debug, Clone)]
pub struct Edge {
    /// Source node.
    pub source: usize,

    /// Destination node.
    pub destination: usize,

    /// Weight of the edge.
    pub weight: i32,
}

/// Bellman-Ford Algorithm.
/// Compute shortest paths from a single source vertex to all of the other vertices in a weighted digraph.
#[derive(Debug, Clone)]
pub struct BellmanFordAlgorithm {
    /// Total number of vertices in the graph.
    pub total_vertices: usize,

    /// Edges in the graph.
    pub edges: Vec<Edge>,
}

impl Default for BellmanFordAlgorithm {
    /// Create a new default instance of Bellman-Ford Algorithm.
    ///
    /// # Returns
    ///
    /// New default instance of Bellman-Ford Algorithm.
    fn default() -> Self {
        Self::new()
    }
}

impl BellmanFordAlgorithm {
    /// Create a new instance of Bellman-Ford Algorithm.
    ///
    /// # Returns
    ///
    /// New instance of Bellman-Ford Algorithm.
    pub fn new() -> Self {
        BellmanFordAlgorithm {
            total_vertices: 0,
            edges: Vec::new(),
        }
    }

    /// Set a node's edges to the graph.
    ///
    /// # Arguments
    ///
    /// - `source`: Source node.
    /// - `edges`: Edges of the source node.
    pub fn add_edge(&mut self, source: usize, edges: Vec<(usize, i32)>) {
        if edges.is_empty() {
            self.total_vertices = self.total_vertices.max(source + 1);
            return;
        }

        for (destination, weight) in edges {
            self.edges.push(Edge {
                source,
                weight,
                destination,
            });

            self.total_vertices = self.total_vertices.max(source + 1).max(destination + 1);
        }
    }

    /// Set multiple nodes' edges to the graph.
    ///
    /// # Arguments
    ///
    /// - `nodes`: Vector of tuples where each tuple contains a node and its associated edges.
    pub fn add_edges(&mut self, nodes: Vec<(usize, Vec<(usize, i32)>)>) {
        for (source, edges) in nodes {
            self.add_edge(source, edges);
        }
    }
}

impl GraphAlgorithm for BellmanFordAlgorithm {
    /// Type of node.
    type Node = isize;

    /// Type of weight.
    type Weight = Vec<i32>;

    /// Run Bellman-Ford Algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: Starting node.
    ///
    /// # Returns
    ///
    /// Result containing a vector of shortest paths, or an error if applicable.
    fn run(&self, start: Option<Self::Node>) -> Result<Self::Weight, GraphError> {
        let start = start.ok_or(GraphError::MissingStartNode)?;

        let mut distances = vec![i32::MAX; self.total_vertices];
        distances[start as usize] = 0;

        for _ in 0..self.total_vertices - 1 {
            let mut is_distance_updated = false;

            for edge in &self.edges {
                if distances[edge.source] != i32::MAX {
                    let new_distance = distances[edge.source] + edge.weight;

                    if new_distance < distances[edge.destination] {
                        distances[edge.destination] = new_distance;
                        is_distance_updated = true;
                    }
                }
            }

            if !is_distance_updated {
                break;
            }
        }

        for edge in &self.edges {
            if distances[edge.source] != i32::MAX {
                let new_distance = distances[edge.source] + edge.weight;

                if new_distance < distances[edge.destination] {
                    return Err(GraphError::NegativeWeightCycle);
                }
            }
        }

        Ok(distances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let algorithm = BellmanFordAlgorithm::new();
        let algorithm_default = BellmanFordAlgorithm::default();

        assert_eq!(algorithm.edges.len(), 0);
        assert_eq!(algorithm_default.edges.len(), 0);
    }

    #[test]
    fn test_missing_start_node() {
        let algorithm = BellmanFordAlgorithm::new();

        assert_eq!(algorithm.run(None), Err(GraphError::MissingStartNode));
    }

    #[test]
    fn test_run() {
        let mut algorithm = BellmanFordAlgorithm {
            total_vertices: 50,
            edges: Vec::new(),
        };

        let edges = vec![
            (0, 1, 4),
            (0, 2, 1),
            (1, 2, 2),
            (1, 3, 5),
            (2, 3, 8),
            (2, 4, 2),
            (3, 5, 3),
            (4, 5, 4),
            (5, 6, 1),
            (6, 7, 3),
            (7, 8, 2),
            (8, 9, 6),
            (9, 10, 2),
            (10, 11, 1),
            (11, 12, 3),
            (12, 13, 4),
            (13, 14, 2),
            (14, 15, 1),
            (15, 16, 5),
            (16, 17, 1),
            (17, 18, 2),
            (18, 19, 3),
            (19, 20, 6),
            (20, 21, 2),
            (21, 22, 1),
            (22, 23, 3),
            (23, 24, 4),
            (24, 25, 2),
            (25, 26, 1),
            (26, 27, 5),
            (27, 28, 1),
            (28, 29, 2),
            (29, 30, 3),
            (30, 31, 6),
            (31, 32, 2),
            (32, 33, 1),
            (33, 34, 3),
            (34, 35, 4),
            (35, 36, 2),
            (36, 37, 1),
            (37, 38, 5),
            (38, 39, 1),
            (39, 40, 2),
            (40, 41, 3),
            (41, 42, 6),
            (42, 43, 2),
            (43, 44, 1),
            (44, 45, 4),
            (45, 46, 1),
            (46, 47, 2),
            (47, 48, 3),
            (48, 49, 1),
        ];

        for (source, destination, weight) in edges {
            algorithm.add_edge(source, vec![(destination, weight)]);
        }

        let result = algorithm.run(Some(0)).unwrap();

        assert_eq!(result[0], 0);

        for &distance in &result {
            assert!(distance >= 0 || distance == i32::MAX);
        }

        for item in result.iter().take(10).skip(1) {
            assert!(item < &i32::MAX);
        }
    }

    #[test]
    fn test_run_single_node_graph() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0]);
    }

    #[test]
    fn test_run_simple_graph_no_negative_edges() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 4), (2, 3)]);
        algorithm.add_edge(1, vec![(2, 1), (3, 2)]);
        algorithm.add_edge(2, vec![(3, 5)]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 4, 3, 6]);
    }

    #[test]
    fn test_run_graph_with_negative_edge() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 4), (2, 3)]);
        algorithm.add_edge(1, vec![(2, -2), (3, 2)]);
        algorithm.add_edge(2, vec![(3, 3)]);

        assert_eq!(algorithm.run(Some(0)).unwrap(), vec![0, 4, 2, 5]);
    }

    #[test]
    fn test_run_graph_with_no_edges() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![]);
        algorithm.add_edge(1, vec![]);
        algorithm.add_edge(2, vec![]);
        algorithm.add_edge(3, vec![]);

        assert_eq!(
            algorithm.run(Some(0)).unwrap(),
            vec![0, i32::MAX, i32::MAX, i32::MAX]
        );
    }

    #[test]
    fn test_run_run_from_different_start_node() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 4), (2, 3)]);
        algorithm.add_edge(1, vec![(2, 1), (3, 2)]);
        algorithm.add_edge(2, vec![(3, 5)]);

        assert_eq!(algorithm.run(Some(1)).unwrap(), vec![i32::MAX, 0, 1, 2]);
    }

    #[test]
    fn test_run_disconnected_graph() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 4)]);
        algorithm.add_edge(2, vec![(3, 5)]);

        assert_eq!(
            algorithm.run(Some(0)).unwrap(),
            vec![0, 4, i32::MAX, i32::MAX]
        );
    }

    #[test]
    fn test_run_graph_with_negative_weight_cycle() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edges(vec![
            (0, vec![(1, 1)]),
            (1, vec![(2, -1)]),
            (2, vec![(0, -1)]),
        ]);

        assert_eq!(algorithm.run(Some(0)), Err(GraphError::NegativeWeightCycle));
    }

    #[test]
    fn test_run_early_exit_no_updates() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 2)]);
        algorithm.add_edge(1, vec![(2, 3)]);
        algorithm.add_edge(2, vec![(3, 4)]);
        algorithm.add_edge(3, vec![(4, 1)]);

        let result = algorithm.run(Some(0)).unwrap();

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 5);
        assert_eq!(result[3], 9);
        assert_eq!(result[4], 10);
    }

    #[test]
    fn test_run_early_exit_with_no_negative_cycle() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 2)]);
        algorithm.add_edge(1, vec![(2, 3)]);
        algorithm.add_edge(2, vec![(3, -5)]);
        algorithm.add_edge(3, vec![(4, 1)]);

        let result = algorithm.run(Some(0)).unwrap();

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 5);
        assert_eq!(result[3], 0);
        assert_eq!(result[4], 1);
    }

    #[test]
    fn test_run_early_exit_with_negative_cycle() {
        let mut algorithm = BellmanFordAlgorithm::new();
        algorithm.add_edge(0, vec![(1, 1)]);
        algorithm.add_edge(1, vec![(2, -2)]);
        algorithm.add_edge(2, vec![(0, -1)]); // Negative cycle here

        assert_eq!(algorithm.run(Some(0)), Err(GraphError::NegativeWeightCycle));
    }
}
