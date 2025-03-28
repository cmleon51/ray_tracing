/// module implementing a Camera struct
mod camera;
/// module implementing a Ray struct
mod ray;
/// module implementing the Vec3 struct and all of the math behind it
mod vec3;
/// module implementing all of the objects our scene can render
mod objects;
/// module implementing all of the lights our scene can use
mod lights;

pub use camera::{Camera, ViewportAngles};
pub use ray::Ray;
pub use vec3::Vec3;
pub use objects::{Sphere, create_objects_vec};
pub use lights::{Light, PointLight, DirectionalLight, AmbientLight};
