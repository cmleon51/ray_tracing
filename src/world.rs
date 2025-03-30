/// module implementing the Vec3 struct and all of the math behind it
mod vec3;
/// module implementing a Ray struct
mod ray;
/// module implementing a Camera struct
mod camera;
/// module implementing all of the objects our scene can render 
mod objects;
/// module implementing all of the lights our scene can use
mod lights;

pub use camera::{Camera, ViewportAngles};
pub use lights::{AmbientLight, DirectionalLight, Light, PointLight};
pub use objects::{Object, Material, Sphere};
pub use ray::Ray;
pub use vec3::Vec3;
