//! Triplet mining for contrastive/metric learning — pure Rust port.
//!
//! Provides triplet data structures, mining strategies (random, hard, semi-hard,
//! domain-aware), text similarity (Jaccard, shingle-based), quality filtering,
//! and evaluation metrics.

pub mod triplet;
pub mod strategies;
pub mod filters;
pub mod distance;
pub mod evaluation;

pub use triplet::Triplet;
pub use strategies::{MiningStrategy, compute_similarity, select_negatives};
pub use filters::QualityFilter;
