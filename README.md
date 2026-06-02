# algtop-rs

> Algebraic topology in Rust. Homology, cohomology, and the shape of spaces.

## Features

- **Simplicial complexes** — build and compute with simplicial complexes
- **Homology** — simplicial and singular homology groups
- **Cohomology** — cohomology rings and operations
- **Mayer-Vietoris** — decomposition sequences
- **Poincaré duality** — for closed manifolds
- **Homotopy groups** — fundamental group and higher homotopy
- **CW complexes** — cell complexes and their topology
- **Euler characteristic** — classical and Betti-number based
- **Network topology** — model graphs as simplicial complexes

## Usage

```toml
cargo add algtop-rs
```

```rust
use algtop_rs::simplicial::*;

let mut sc = SimplicialComplex::new();
sc.add_simplex(&[0, 1, 2]);
let homology = sc.homology(3);
for (dim, group) in homology.iter().enumerate() {
    println!("H_{} = {:?}", dim, group);
}
```

## License

MIT OR Apache-2.0
