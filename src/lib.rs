//! # algtop-rs
//!
//! Algebraic topology in Rust: simplicial/singular homology, cohomology,
//! Mayer-Vietoris sequence, Poincaré duality, homotopy groups, CW complexes,
//! and network topology analysis.

pub mod chain_complex;
pub mod simplicial;
pub mod singular;
pub mod mayer_vietoris;
pub mod homotopy;
pub mod cw_complex;
pub mod cohomology;
pub mod poincare;
pub mod euler;
pub mod network;

pub use chain_complex::*;
pub use simplicial::*;
pub use singular::*;
pub use mayer_vietoris::*;
pub use homotopy::*;
pub use cw_complex::*;
pub use cohomology::*;
pub use poincare::*;
pub use euler::*;
pub use network::*;
