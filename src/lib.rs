#![warn(missing_docs)]
//! This crate has been made to abstract all of the necessary components
//! to implement the ['ray tracing in one week'] book
//!
//! ['ray tracing in one week']: https://raytracing.github.io/books/RayTracingInOneWeekend.html

/// This module abstracts the final output image giving the user the necessary components to create
/// the output file
pub mod image;

/// This module abstracts the world in which the ray tracing is done
pub mod world;
