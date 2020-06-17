//! # permute
//! Generate permutations of a slice in a memory-efficient and deterministic manner, using
//! [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm).
pub mod arbitrary_tandem_control_iter;
mod permutations;
mod permute_iter;

pub use permutations::permute;
pub use permute_iter::permutations_of;
