#![allow(warnings)]
#![warn(missing_docs)]
//! This crate has been made to abstract all of the necessary components
//! to implement the ['ray tracing in one week'] book
//!
//! ['ray tracing in one week']: https://raytracing.github.io/books/RayTracingInOneWeekend.html

/// This module abstracts the canvas on which the rendered image will be displayed + it's single
/// components
pub mod canvas;

/// This module abstracts the world in which the ray tracing is done
pub mod world;

///This module gives the user of this library some utilities to easily use the library
pub mod ray_utils;

/// This module holds everything we have created in a nice package ready to be used
pub mod ray_tracer;
