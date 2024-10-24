mod dijkstra;

fn main() {
    // Run the Dijkstra example
    dijkstra::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // Just test that the main function runs without panicking
        main();
    }
}
