use crate::canvas::RGB;
use crate::ray_tracer::RayTracer;
use crate::world::objects::Object;
use crate::world::{ObjectRayIntersection, Ray, Vec3};

/// trough this trait we can implement every type of light we may need for our ray traced world
pub trait Light {
    /// this function should return the current_object's color at the specified ray and t
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        other_objects: &Vec<Box<dyn Object>>,
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB;
}

/// module to implement an ambient light
mod ambient_light;
/// module to implement a directional light
mod directional_light;
/// module to implement a point light
mod point_light;

// extracting everything we may need
pub use ambient_light::AmbientLight;
pub use directional_light::DirectionalLight;
pub use point_light::PointLight;
