mod bellman_ford;
mod dijkstra;

fn main() {
    // Run the Dijkstra example
    dijkstra::run();

    // Run the Bellman-Ford example
    bellman_ford::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
