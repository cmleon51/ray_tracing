/// module implementing a Camera struct
mod camera;
/// module implementing all of the lights our scene can use
mod lights;
/// module implementing the materials our scene can render
mod materials;
/// module implementing all of the objects our scene can render
mod objects;
/// module implementing a Ray struct
mod ray;
/// module implementing the Vec3 struct and all of the math behind it
mod vec3;

// extracting everything that is useful
pub use camera::{Camera, ViewportAngles};
pub use lights::{Light, Lights};
pub use materials::{Material, MaterialBuilder};
pub use objects::{Object, ObjectRayIntersection, Objects};
pub use ray::Ray;
pub use vec3::Vec3;
