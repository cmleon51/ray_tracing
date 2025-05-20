use std::f64;

use crate::canvas::RGB;
use crate::world::objects::{Material, Object};
use crate::world::{Ray, Vec3};

/// object to abstract the bounding box of a Panel
///
/// Trough the use of this object we can create a bounding box and check if a specific vector is
/// inside or outside
struct BoundingBox {
    top_right: Vec3,
    top_left: Vec3,
    bottom_right: Vec3,
    bottom_left: Vec3,
}

impl BoundingBox {
    pub fn new(bounding_center: Vec3, bounding_width: f64, bounding_height: f64) -> Self {
        return Self {
            top_right: bounding_center + Vec3::new(bounding_width, bounding_height, 0.0),
            top_left: bounding_center + Vec3::new(-bounding_width, bounding_height, 0.0),
            bottom_right: bounding_center + Vec3::new(bounding_width, -bounding_height, 0.0),
            bottom_left: bounding_center + Vec3::new(-bounding_width, -bounding_height, 0.0),
        };
    }

    pub fn is_vec_inside(&self, vec_to_check: Vec3) -> bool {
        let vec_check_x = *vec_to_check.get_x();
        let vec_check_y = *vec_to_check.get_y();
        let vec_check_z = *vec_to_check.get_z();

        return vec_check_x < *self.top_right.get_x()
            && vec_check_y < *self.top_right.get_y()
            && vec_check_x > *self.top_left.get_x()
            && vec_check_y < *self.top_left.get_y()
            && vec_check_x < *self.bottom_right.get_x()
            && vec_check_y > *self.bottom_right.get_y()
            && vec_check_x > *self.bottom_left.get_x()
            && vec_check_y > *self.bottom_right.get_y();
    }
}

/// object to abstract a panel in our ray traced world
///
/// Trough the use of this object we can create a panel in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Panel {
    panel_origin: Vec3,
    panel_normal: Vec3,
    bounding_box: BoundingBox,
    material: Material,
}

impl Panel {
    pub fn new(
        panel_origin: Vec3,
        panel_normal: Vec3,
        panel_width: f64,
        panel_height: f64,
        material: Material,
    ) -> Self {
        return Self {
            panel_origin,
            panel_normal,
            bounding_box: BoundingBox::new(panel_origin, panel_width, panel_height),
            material,
        };
    }
}

impl Object for Panel {
    fn is_object_hit(&self, ray: &Ray) -> Option<f64> {
        let panel_normal = self.get_normal(Vec3::new(0.0, 0.0, 0.0))?;
        let discriminant = ray.get_direction().dot_product(&panel_normal);

        if discriminant == 0.0 {
            return None;
        }

        let t = ((self.panel_origin - (*ray.get_direction())).dot_product(&panel_normal))
            / discriminant;

        // it's certinately not the best way to check if a ray is inside the width and height of
        // the box but it's the one i've come up with
        if !self
            .bounding_box
            .is_vec_inside(ray.calculate_ray_position(t))
        {
            return None;
        }

        return Some(t);
    }

    fn get_normal(&self, point: Vec3) -> Option<Vec3> {
        return Some(self.panel_normal);
    }

    fn get_material(&self) -> &Material {
        return &(self.material);
    }

    fn get_color(&self, point: Vec3) -> RGB {
        return *self.get_material().get_color();
    }
}
