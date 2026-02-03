# ac-lib

[![Crates.io](https://img.shields.io/crates/v/ac-lib.svg)](https://crates.io/crates/ac-lib)
[![Documentation](https://docs.rs/ac-lib/badge.svg)](https://docs.rs/ac-lib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for competitive programming on AtCoder.

## Features

- 🔢 **Math**: GCD/LCM, Prime numbers, ModInt
- 🗺️ **Graph**: DFS, BFS, Dijkstra, Union-Find
- 📊 **Data Structures**: Segment Tree, Fenwick Tree

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ac-lib = "0.1.0"
```

## Usage

### Math

```rust
use ac_lib::math::{gcd, is_prime, ModInt};

// GCD
let result = gcd(48, 18);
println!("{}", result); // 6

// Prime check
println!("{}", is_prime(17)); // true

// Modular arithmetic
let a = ModInt::new(1000000000, 1000000007);
let b = ModInt::new(1000000000, 1000000007);
let c = a.add(&b);
println!("{}", c.value()); // 1999999993
```

### Graph

```rust
use ac_lib::graph::{dfs, bfs, dijkstra, Edge, UnionFind};

// DFS
let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
let mut visited = vec![false; 4];
let order = dfs(&graph, 0, &mut visited);

// Union-Find
let mut uf = UnionFind::new(5);
uf.union(0, 1);
uf.union(1, 2);
assert!(uf.connected(0, 2));
assert_eq!(uf.size(0), 3);
```

### Data Structures

```rust
use ac_lib::structure::{SegmentTree, FenwickTree};

// Segment Tree
let arr = vec![1, 3, 5, 7, 9];
let mut segtree = SegmentTree::new(&arr);
println!("{}", segtree.query(1, 4)); // 15

// Fenwick Tree
let mut ft = FenwickTree::from_vec(&arr);
println!("{}", ft.range_sum(1, 3)); // 15
```

## Testing

```bash
cargo test
Or generate locally:

```bash
cargo doc --open
```

## Testing

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Issues and Pull Requests are welcome!
