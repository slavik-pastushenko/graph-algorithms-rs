mod bellman_ford;
mod dijkstra;
mod floyd_warshall;

fn main() {
    // Run the Dijkstra example
    dijkstra::run();

    // Run the Bellman-Ford example
    bellman_ford::run();

    // Run the Floyd-Warshall example
    floyd_warshall::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
