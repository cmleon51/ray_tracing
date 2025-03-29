use crate::image::RGB;
use crate::world::objects::Object;
use crate::world::{Ray, Vec3};

/// trough this trait we can implement every type of light we may need for our ray traced world
pub trait Light {
    /// this function should return the current_object's color at the specified ray and t
    fn compute_color(
        &self,
        ray: &Ray,
        t: f64,
        viewing_vector: Vec3,
        current_object: &Box<dyn Object>,
        other_objects: &Vec<Box<dyn Object>>,
        other_lights: &Vec<Box<dyn Light>>,
        light_bounces: u8,
    ) -> RGB;
}

/// module to implement an ambient light
mod ambient_light;
/// module to implement a directional light
mod directional_light;
/// module to implement a point light
mod point_light;

pub use ambient_light::AmbientLight;
pub use directional_light::DirectionalLight;
pub use point_light::PointLight;
