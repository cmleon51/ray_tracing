#![allow(warnings)]
#![warn(missing_docs)]
//! This crate has been made to abstract all of the necessary components
//! to implement the ray tracing algorithm

/// This module abstracts the canvas on which the rendered image will be displayed + it's single
/// components
mod canvas;

/// This module abstracts the world in which the ray tracing is done
mod world;

/// This module holds everything we have created in a nice package ready to be used
mod ray_tracer;

// extract everything that is needed to create the ray tracing
pub use canvas::RGB;
pub use world::{AmbientLight, DirectionalLight, Material, PointLight, Sphere, Triangle, Vec3};
pub use ray_tracer::RayTracer;
