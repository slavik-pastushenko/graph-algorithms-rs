use graph_algorithms::{BellmanFordAlgorithm, GraphAlgorithm};

pub fn run() -> Vec<i32> {
    let mut algorithm = BellmanFordAlgorithm::new();
    algorithm.add_edge(0, vec![(1, 4), (2, 3)]);
    algorithm.add_edge(1, vec![(2, 1), (3, 2)]);
    algorithm.add_edge(2, vec![(3, 5)]);

    algorithm.run(0).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_run() {
        assert_eq!(run(), vec![0, 4, 3, 6]);
    }
}
