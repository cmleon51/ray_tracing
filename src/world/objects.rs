// I'm not sure which version would be better
use crate::world::Ray;
//use super::Ray;
//use super::Vec3;

/// trait for implementing the "is_object_hit" function
pub trait Hit {
    /// this method should do all of the necessary calculations to check if a ray hits an object
    /// and return the `t` ray parameter that is closest to the ray
    fn is_object_hit(&self, ray: &Ray) -> f64;
}

/// module implementing a sphere in our ray traced world
mod sphere;

// TODO: find a better way to extract the modules for the lib
pub use sphere::Sphere;

/// function to retrieve a Vec allowing the user to store any object that implements `Hit`
pub fn create_objects_vec() -> Vec<Box<dyn Hit>> {
    return vec![];
}
