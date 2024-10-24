use graph_algorithms::{Dijkstra, GraphAlgorithm};

pub fn run() {
    let mut dijkstra = Dijkstra::new();
    dijkstra.set_nodes(vec![
        (0, vec![(1, 1), (2, 4)]),
        (1, vec![(2, 2)]),
        (2, vec![]),
    ]);

    let result = dijkstra.run(0);

    println!("{:?}", result);
}
