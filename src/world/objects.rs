// I'm not sure which version would be better
use crate::image::RGB;
use crate::world::Ray;
use crate::world::Vec3;
//use super::Ray;
//use super::Vec3;

/// An object rappresenting the material properties of our objects
#[derive(Debug, Copy, Clone)]
pub struct Material {
    color: RGB,
    reflectiveness: Option<f64>,
    specularity: Option<f64>,
}

impl Material {
    pub fn new(color: RGB, reflectiveness: Option<f64>, specularity: Option<f64>) -> Self {
        //let specularity = match specularity {
        //    None => None,
        //    Some(specularity) => Some(specularity.clamp(0.0, 1.0))
        //};
        let reflectiveness = match reflectiveness {
            None => None,
            Some(reflectiveness) => Some(reflectiveness.clamp(0.0, 1.0)),
        };

        return Self {
            color,
            reflectiveness,
            specularity,
        };
    }

    /// retrieves the material's specularity value
    pub fn get_specularity(&self) -> &Option<f64> {
        return &self.specularity;
    }

    /// retrieves the material's reflectiveness
    pub fn get_reflectiveness(&self) -> &Option<f64> {
        return &self.reflectiveness;
    }

    /// retrieves the material's color
    pub fn get_color(&self) -> &RGB {
        return &self.color;
    }
}

/// trait for implementing the necessary functions to make a type a `Object`
pub trait Object {
    /// this method should do all of the necessary calculations to check if a ray hits an object
    /// and return the `t` ray parameter that is closest to the ray
    fn is_object_hit(&self, ray: &Ray) -> f64;

    /// this method should retun a unit vector of the normal of the object based upon a point in
    /// space that is on the surface of the object (the value could be None if the point given is
    /// not on the surface
    fn get_normal(&self, point: Vec3) -> Option<Vec3>;

    /// this methos should return the object's material
    fn get_material(&self) -> &Material;

    ///// this method should return the color of the object
    //fn get_color(&self) -> &RGB;
    //
    ///// this methos should return the specularity of the current object
    //fn get_specularity(&self) -> Option<f64>;
    //
    ///// this method should return the reflection of the current object
    //fn get_reflection(&self) -> Option<f64>;
}

/// module implementing a sphere in our ray traced world
mod sphere;

// TODO: find a better way to extract the modules for the lib or create a function for every object
pub use sphere::Sphere;
