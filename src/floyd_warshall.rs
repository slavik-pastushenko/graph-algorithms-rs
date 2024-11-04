use crate::{GraphAlgorithm, GraphError};

/// Floyd-Warshall Algorithm.
/// Compute shortest paths between all pairs of vertices in a weighted graph.
#[derive(Debug, Clone)]
pub struct FloydWarshallAlgorithm {
    /// Total number of nodes in the graph.
    pub total_nodes: usize,

    /// Edges in the graph.
    pub edges: Vec<(usize, usize, i32)>,
}

impl Default for FloydWarshallAlgorithm {
    /// Create a new default instance of Floyd-Warshall Algorithm.
    ///
    /// # Returns
    ///
    /// New default instance of Floyd-Warshall Algorithm.
    fn default() -> Self {
        Self::new()
    }
}

impl FloydWarshallAlgorithm {
    /// Create a new instance of Floyd-Warshall Algorithm.
    ///
    /// # Returns
    ///
    /// New instance of Floyd-Warshall Algorithm.
    pub fn new() -> Self {
        Self {
            total_nodes: 0,
            edges: Vec::new(),
        }
    }

    /// Set a single edge to the graph.
    ///
    /// # Arguments
    ///
    /// - `source`: Source node.
    /// - `target`: Target node.
    /// - `weight`: Weight of the edge.
    pub fn set_edge(&mut self, source: usize, target: usize, weight: i32) {
        self.edges.push((source, target, weight));
        self.total_nodes = self.total_nodes.max(source + 1).max(target + 1);
    }

    /// Set multiple nodes' edges to the graph.
    ///
    /// # Arguments
    ///
    /// - `nodes`: Vector of tuples where each tuple contains a node and its associated edges.
    pub fn set_edges(&mut self, nodes: Vec<(usize, Vec<(usize, i32)>)>) {
        for (source, edges) in nodes {
            for (target, weight) in edges {
                self.set_edge(source, target, weight);
            }
        }
    }

    /// Set the total number of nodes in the graph.
    ///
    /// # Arguments
    ///
    /// - `total`: Total number of nodes in the graph.
    pub fn set_total_nodes(&mut self, total: usize) {
        self.total_nodes = self.total_nodes.max(total);
    }
}

impl GraphAlgorithm for FloydWarshallAlgorithm {
    /// Type of node.
    type Node = usize;

    /// Type of weight.
    type Weight = Vec<Vec<i32>>;

    /// Run Floyd-Warshall algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: Starting node. This is not used in Floyd-Warshall algorithm.
    ///
    /// # Returns
    ///
    /// Result containing a vector of shortest paths, or an error if applicable.
    fn run(&self, _start: Option<Self::Node>) -> Result<Self::Weight, GraphError> {
        let mut distances = vec![vec![i32::MAX; self.total_nodes]; self.total_nodes];

        for &(u, v, w) in &self.edges {
            distances[u][v] = w;
        }

        for (v, row) in distances.iter_mut().enumerate().take(self.total_nodes) {
            row[v] = 0;
        }

        for k in 0..self.total_nodes {
            for i in 0..self.total_nodes {
                for j in 0..self.total_nodes {
                    if distances[i][k] != i32::MAX && distances[k][j] != i32::MAX {
                        distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                    }
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
        let algorithm = FloydWarshallAlgorithm::new();
        let algorithm_default = FloydWarshallAlgorithm::default();

        assert_eq!(algorithm.edges.len(), 0);
        assert_eq!(algorithm_default.edges.len(), 0);
    }

    #[test]
    fn test_run() {
        let mut algorithm = FloydWarshallAlgorithm::new();

        algorithm.set_edge(0, 1, 4);
        algorithm.set_edge(1, 2, 1);
        algorithm.set_edge(0, 2, 7);
        algorithm.set_edge(2, 3, 3);
        algorithm.set_edge(3, 4, 2);
        algorithm.set_edge(4, 5, 1);
        algorithm.set_edge(5, 6, 6);
        algorithm.set_edge(0, 6, 15);
        algorithm.set_edge(1, 4, 8);
        algorithm.set_edge(2, 5, 12);
        algorithm.set_edge(3, 6, 7);
        algorithm.set_edge(4, 2, 5);
        algorithm.set_edge(5, 0, 10);
        algorithm.set_edge(6, 1, 11);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][0], 0);
        assert_eq!(result[0][1], 4);
        assert_eq!(result[0][2], 5);
        assert_eq!(result[0][3], 8);
        assert_eq!(result[0][4], 10);
        assert_eq!(result[0][5], 11);
        assert_eq!(result[0][6], 15);
        assert_eq!(result[1][3], 4);
        assert_eq!(result[1][4], 6);
        assert_eq!(result[2][5], 6);
        assert_eq!(result[3][5], 3);
        assert_eq!(result[4][6], 7);
    }

    #[test]
    fn test_run_single_node() {
        let mut algorithm = FloydWarshallAlgorithm::new();
        algorithm.set_edge(0, 0, 0);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][0], 0);
    }

    #[test]
    fn test_run_two_nodes_one_edge() {
        let mut algorithm = FloydWarshallAlgorithm::new();
        algorithm.set_edge(0, 1, 5);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][1], 5);
        assert_eq!(result[1][0], i32::MAX);
    }

    #[test]
    fn test_run_negative_weights_without_cycle() {
        let mut algorithm = FloydWarshallAlgorithm::new();
        algorithm.set_edges(vec![
            (0, vec![(1, -2), (2, 4)]),
            (1, vec![(2, 1)]),
            (0, vec![(2, 4)]),
        ]);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][1], -2);
        assert_eq!(result[0][2], -1);
    }

    #[test]
    fn test_run_complete_graph() {
        let mut algorithm = FloydWarshallAlgorithm::new();
        algorithm.set_edge(0, 1, 1);
        algorithm.set_edge(0, 2, 2);
        algorithm.set_edge(1, 2, 1);
        algorithm.set_edge(1, 0, 3);
        algorithm.set_edge(2, 0, 4);
        algorithm.set_edge(2, 1, 5);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][1], 1);
        assert_eq!(result[0][2], 2);
        assert_eq!(result[1][2], 1);
    }

    #[test]
    fn test_run_disconnected_nodes() {
        let mut algorithm = FloydWarshallAlgorithm::new();
        algorithm.set_edge(0, 1, 3);
        algorithm.set_edge(1, 2, 4);
        algorithm.set_total_nodes(4);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][3], i32::MAX);
        assert_eq!(result[3][3], 0);
    }

    #[test]
    fn test_run_zero_weight_cycle() {
        let mut algorithm = FloydWarshallAlgorithm::new();
        algorithm.set_edge(0, 1, 2);
        algorithm.set_edge(1, 2, -3);
        algorithm.set_edge(2, 0, 1);

        let result = algorithm.run(None).unwrap();

        assert_eq!(result[0][1], 2);
        assert_eq!(result[1][2], -3);
        assert_eq!(result[2][0], 1);
        assert_eq!(result[0][2], -1);
    }
}
