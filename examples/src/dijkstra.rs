use graph_algorithms::{DijkstraAlgorithm, GraphAlgorithm};

pub fn run() -> Vec<usize> {
    let mut algorithm = DijkstraAlgorithm::new();
    algorithm.set_nodes(vec![
        (0, vec![(1, 1), (2, 4)]),
        (1, vec![(2, 2)]),
        (2, vec![]),
    ]);

    algorithm.run(Some(0)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_run() {
        assert_eq!(run(), vec![0, 1, 3]);
    }
}
