use crate::world::Vec3;
use crate::world::objects::Object;

/// trough this trait we can implement every type of light we may need for our ray traced world
pub trait Light {
    /// this function should return the intensity of the light based upon a point in space and the
    /// normal of the object given
    fn get_intensity(&self, point: Vec3, viewing_vector: Vec3, object: &Box<dyn Object>) -> Option<f64>;
}

/// module to implement a point light
mod point_light;
/// module to implement a directional light
mod directional_light;
/// module to implement an ambient light
mod ambient_light;

pub use point_light::PointLight;
pub use directional_light::DirectionalLight;
pub use ambient_light::AmbientLight;
