use super::Object;
use crate::world::{Ray, Vec3};
use crate::image::RGB;

/// object to abstract a sphere in our ray traced world
///
/// Trough the use of this object we can create a sphere in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Sphere {
    // there is no necessity to have private fields on this object
    position: Vec3,
    radius: f64,
    color: RGB,
    specularity: Option<f64>,
}

impl Sphere {
    /// function to create a new sphere
    /// even thogh the fields are public i like to have a function to create a new sphere
    pub fn new(position: Vec3, radius: f64, red: u8, green: u8, blue: u8, specularity: Option<f64>) -> Self {
        return Self { 
            position,
            radius,
            color: RGB::new(red, green, blue),
            specularity,
        };
    }
}

impl Object for Sphere {
    fn is_object_hit(&self, ray: &Ray) -> f64 {
        let oc = (*ray.get_position()) - self.position;
        let a = ray.get_direction().dot_product(ray.get_direction());
        let b = ray.get_direction().dot_product(&oc) * 2.0;
        let c = oc.dot_product(&oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        return f64::min(t1, t2); // i think that the syntax is more readable like this
    }

    // TODO: check if the point is inside of the sphere in a better way since we have to account
    // for float number's shenanigans
    fn get_normal(&self, point: Vec3) -> Option<Vec3> {
        //if (point - self.position).dot_product(&(point - self.position)) != (self.radius * self.radius) {
        //    return None;
        //}

        return Some(*((point - self.position).make_unit()));
    }

    fn get_color(&self) -> &RGB {
        return &self.color;
    }

    fn get_specularity(&self) -> Option<f64> {
        return self.specularity;
    }
}
