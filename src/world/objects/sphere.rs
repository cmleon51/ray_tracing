use super::Hit;
use crate::world::{Ray, Vec3};

/// object to abstract a sphere in our ray traced world
///
/// Trough the use of this object we can create a sphere in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Sphere {
    // there is no necessity to have private fields on this object
    pub position: Vec3,
    pub radius: f64,
}

impl Sphere {
    /// function to create a new sphere
    /// even thogh the fields are public i like to have a function to create a new sphere
    pub fn new(position: Vec3, radius: f64) -> Self {
        return Self { position, radius };
    }
}

impl Hit for Sphere {
    fn is_object_hit(&self, ray: &Ray) -> f64 {
        let oc = (*ray.get_position()) - self.position;
        let a = ray.get_direction().dot_product(ray.get_direction());
        let b = ray.get_direction().dot_product(&oc) * 2.0;
        let c = oc.dot_product(&oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        return f64::max(t1, t2); // i think that the syntax is more readable like this
    }
}
