use graph_algorithms::{FloydWarshallAlgorithm, GraphAlgorithm};

pub fn run() -> Vec<Vec<i32>> {
    let mut algorithm = FloydWarshallAlgorithm::new();
    algorithm.add_edge(0, 1, 1);
    algorithm.add_edge(0, 2, 2);
    algorithm.add_edge(1, 2, 1);
    algorithm.add_edge(1, 0, 3);
    algorithm.add_edge(2, 0, 4);
    algorithm.add_edge(2, 1, 5);

    algorithm.run(None).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_run() {
        let result = run();

        assert_eq!(result[0][1], 1);
        assert_eq!(result[0][2], 2);
        assert_eq!(result[1][2], 1);
    }
}
