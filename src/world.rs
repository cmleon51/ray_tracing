/// module implementing a Camera struct
mod camera;
/// module implementing all of the lights our scene can use
mod lights;
/// module implementing all of the objects our scene can render
mod objects;
/// module implementing a Ray struct
mod ray;
/// module implementing the Vec3 struct and all of the math behind it
mod vec3;

pub use camera::{Camera, ViewportAngles};
pub use lights::{AmbientLight, DirectionalLight, Light, PointLight};
pub use objects::{Material, Object, Sphere};
pub use ray::Ray;
pub use vec3::Vec3;
