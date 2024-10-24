# Graph Algorithms

A collection of graph algorithms implemented in Rust. This repository aims to provide efficient and easy-to-understand implementations of various graph algorithms for educational purposes and practical use. 

Contributions are welcome to expand the set of algorithms and improve existing implementations.

## Reference implementation

[![test](https://github.com/slavik-pastushenko/graph-algorithms-rs/actions/workflows/test.yml/badge.svg)](https://github.com/slavik-pastushenko/graph-algorithms-rs/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/slavik-pastushenko/graph-algorithms-rs/graph/badge.svg?token=9EL0F6725A)](https://codecov.io/gh/slavik-pastushenko/graph-algorithms-rs)
[![License](https://img.shields.io/github/license/slavik-pastushenko/graph-algorithms-rs)](https://github.com/slavik-pastushenko/graph-algorithms-rs/blob/main/LICENSE)

## Algorithms

### Dijkstra's Algorithm
Dijkstra's algorithm finds the shortest path from a starting node to all other nodes in a weighted graph. It uses a priority queue to efficiently select the next node with the smallest distance.

### A* Algorithm (TODO)
A* is a pathfinding and graph traversal algorithm that is often used in many fields of computer science due to its completeness, optimality, and optimal efficiency.

### Breadth-First Search (BFS) (TODO)
BFS explores the graph level by level, starting from a given node. It is used for finding the shortest path in an unweighted graph.

### Depth-First Search (DFS) (TODO)
DFS explores as far as possible along each branch before backtracking. It is used for pathfinding and topological sorting.

### Bellman-Ford Algorithm (TODO)
The Bellman-Ford algorithm computes shortest paths from a single source vertex to all of the other vertices in a weighted digraph. It can handle graphs with negative weight edges.

### Floyd-Warshall Algorithm (TODO)
The Floyd-Warshall algorithm finds shortest paths between all pairs of vertices in a weighted graph. It can handle graphs with negative weights but no negative weight cycles.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Contributing

Build the application:

```bash
cargo build
```

Test the application:

```bash
cargo test
```

Run [clippy](https://github.com/rust-lang/rust-clippy):

```bash
cargo clippy --all-targets --all-features --no-deps -- -D warnings
```

Run [lint](https://github.com/rust-lang/rustfmt):

```bash
cargo fmt
```

Generate documentation in HTML format:

```bash
cargo doc --open
```
