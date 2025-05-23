use std::f64;

use super::{Material, Object};
use crate::canvas::RGB;
use crate::world::{Ray, Vec3};

/// object to abstract a sphere in our ray traced world
///
/// Trough the use of this object we can create a sphere in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Sphere {
    // there is no necessity to have private fields on this object
    position: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    /// function to create a new sphere
    /// even thogh the fields are public i like to have a function to create a new sphere
    pub fn new(position: Vec3, radius: f64, material: Material) -> Self {
        return Self {
            position,
            radius,
            material,
        };
    }
}

impl Object for Sphere {
    fn is_object_hit(&self, ray: &Ray) -> Option<f64> {
        let oc = (*ray.get_position()) - self.position;
        let a = ray.get_direction().dot_product(ray.get_direction()); // should always be one
        // but who knows
        let b = ray.get_direction().dot_product(&oc) * 2.0;
        let c = oc.dot_product(&oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        return Some(f64::min(t1, t2)); // i think that the syntax is more readable like this
    }

    // TODO: check if the point is inside of the sphere in a better way since we have to account
    // for float number's shenanigans
    fn get_normal(&self, point: Vec3) -> Option<Vec3> {
        //if (point - self.position).dot_product(&(point - self.position)) != (self.radius * self.radius) {
        //    return None;
        //}

        return Some(*((point - self.position).make_unit()));
    }

    fn get_material(&self) -> &Material {
        return &(self.material);
    }

    fn get_color(&self, point: Vec3) -> RGB {
        let mut final_color = *self.material.get_color();

        if let Some(texture) = self.material.get_texture() {
            let mut point_to_center = point - self.position;
            point_to_center.make_unit();

            let u = 0.5
                + (f64::atan2(*point_to_center.get_z(), *point_to_center.get_x())
                    / (2.0 * f64::consts::PI));
            let v = 0.5 - (f64::asin(*point_to_center.get_y()) / f64::consts::PI);

            final_color += texture.get_color(u, v);
        }

        return final_color;
    }
}
