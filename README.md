# Graph Algorithms

A collection of graph algorithms implemented in Rust. This repository aims to provide efficient and easy-to-understand implementations of various graph algorithms for educational purposes and practical use. 

Contributions are welcome to expand the set of algorithms and improve existing implementations.

## Reference implementation

[![test](https://github.com/slavik-pastushenko/graph-algorithms-rs/actions/workflows/test.yml/badge.svg)](https://github.com/slavik-pastushenko/graph-algorithms-rs/actions/workflows/test.yml)
[![release](https://github.com/slavik-pastushenko/graph-algorithms-rs/actions/workflows/release.yml/badge.svg?event=workflow_dispatch)](https://github.com/slavik-pastushenko/graph-algorithms-rs/actions/workflows/release.yml)
[![docs](https://docs.rs/graph-algorithms-rs/badge.svg)](https://docs.rs/graph-algorithms-rs)
[![crate](https://img.shields.io/crates/v/graph-algorithms-rs.svg)](https://crates.io/crates/graph-algorithms-rs)
![downloads](https://img.shields.io/crates/d/graph-algorithms-rs)
[![codecov](https://codecov.io/gh/slavik-pastushenko/graph-algorithms-rs/graph/badge.svg?token=9EL0F6725A)](https://codecov.io/gh/slavik-pastushenko/graph-algorithms-rs)
[![License](https://img.shields.io/github/license/slavik-pastushenko/graph-algorithms-rs)](https://github.com/slavik-pastushenko/graph-algorithms-rs/blob/main/LICENSE)

| Algorithm       | Description       | Example       |
|-----------------|-------------------|---------------|
| Dijkstra's      | Finds the shortest path from a starting node to all other nodes in a weighted graph. It uses a priority queue to efficiently select the next node with the smallest distance. | ![Dijkstra's](https://upload.wikimedia.org/wikipedia/commons/5/57/Dijkstra_Animation.gif) |
| Bellman-Ford    | Computes shortest paths from a single source vertex to all of the other vertices in a weighted digraph. It can handle graphs with negative weight edges. | ![Bellman-Ford](https://upload.wikimedia.org/wikipedia/commons/thumb/7/77/Bellman%E2%80%93Ford_algorithm_example.gif/440px-Bellman%E2%80%93Ford_algorithm_example.gif) |
| Floyd-Warshall  | Finds shortest paths between all pairs of vertices in a weighted graph. It can handle graphs with negative weights but no negative weight cycles. | ![Floyd-Warshall](https://upload.wikimedia.org/wikipedia/commons/0/0f/Floyd_warshall_gif.gif) |

#### A* Algorithm (TODO)
A* is a pathfinding and graph traversal algorithm that is often used in many fields of computer science due to its completeness, optimality, and optimal efficiency.

#### Breadth-First Search (BFS) (TODO)
BFS explores the graph level by level, starting from a given node. It is used for finding the shortest path in an unweighted graph.

#### Depth-First Search (DFS) (TODO)
DFS explores as far as possible along each branch before backtracking. It is used for pathfinding and topological sorting.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Usage

The `examples` directory contains example implementations of various graph algorithms:

```rust
use graph_algorithms::{DijkstraAlgorithm, GraphAlgorithm};
```

## Features

This crate provides optional features for different algorithms.

By default, all features are enabled. You can customize the features when adding the dependency in your `Cargo.toml`:

```toml
[dependencies]
graph-algorithms-rs = { version = "x.x.x", default-features = false, features = ["dijkstra"] }
```

For a detailed list of available algorithms, refer to the [Reference implementation](#reference-implementation) section.

## Contributing

Build the application:

```bash
cargo build
```

Test the application:

```bash
cargo test
```

Run [clippy](https://github.com/rust-lang/rust-rsppy):

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
