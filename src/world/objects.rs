// I'm not sure which version would be better
use crate::world::Ray;
use crate::image::RGB;
use crate::world::Vec3;
//use super::Ray;
//use super::Vec3;

/// trait for implementing the necessary functions to make a type a `Object`
pub trait Object {
    /// this method should do all of the necessary calculations to check if a ray hits an object
    /// and return the `t` ray parameter that is closest to the ray
    fn is_object_hit(&self, ray: &Ray) -> f64;

    /// this method should retun a unit vector of the normal of the object based upon a point in
    /// space that is on the surface of the object (the value could be None if the point given is
    /// not on the surface
    fn get_normal(&self, point: Vec3) -> Option<Vec3>;

    /// this method should return the color of the object
    fn get_color(&self) -> &RGB;
}

/// module implementing a sphere in our ray traced world
mod sphere;

// TODO: find a better way to extract the modules for the lib or create a function for every object
pub use sphere::Sphere;

/// function to retrieve a Vec allowing the user to store any object that implements `Hit`
pub fn create_objects_vec() -> Vec<Box<dyn Object>> {
    return vec![];
}
