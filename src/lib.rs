//! Triplet mining for contrastive/metric learning — pure Rust port.
//!
//! Provides triplet data structures, mining strategies (random, hard, semi-hard,
//! domain-aware), text similarity (Jaccard, shingle-based), quality filtering,
//! and evaluation metrics.

pub mod distance;
pub mod evaluation;
pub mod filters;
pub mod strategies;
pub mod triplet;

pub use filters::QualityFilter;
pub use strategies::{compute_similarity, select_negatives, MiningStrategy};
pub use triplet::Triplet;
