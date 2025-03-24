/// module implementing the Vec3 struct and all of the math behind it
mod vec3;
/// module implementing a Camera struct
mod camera;
/// module implementing a Ray struct
mod ray;

pub use vec3::Vec3;
pub use camera::{Camera, ViewportAngles};
pub use ray::Ray;
