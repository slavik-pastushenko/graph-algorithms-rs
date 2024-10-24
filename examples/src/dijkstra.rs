use graph_algorithms::{Dijkstra, GraphAlgorithm};

pub fn run() -> Vec<usize> {
    let mut dijkstra = Dijkstra::new();
    dijkstra.set_nodes(vec![
        (0, vec![(1, 1), (2, 4)]),
        (1, vec![(2, 2)]),
        (2, vec![]),
    ]);

    dijkstra.run(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        assert_eq!(run(), vec![0, 1, 3]);
    }
}
