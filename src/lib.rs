//! Data structures and serialization for Dota 2 hero grid configurations.
//!
//! The crate provides three modules:
//! - [`models`] — core data types: `GridMap`, `Grid`, `Category`
//! - [`serials`] — JSON serialization for `GridMap`
//! - [`geometry`] — recursive tree layout system (behind `geometry` feature)

pub mod serials;
pub use serials::*;
pub mod models;
pub use models::*;

#[allow(rust_analyzer::inactive_code)]
#[cfg(feature = "geometry")]
pub mod geometry;
