use crate::canvas::RGB;
use crate::world::objects::{Material, Object};
use crate::world::{Ray, Vec3};

/// object to abstract a panel in our ray traced world
///
/// Trough the use of this object we can create a panel in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Panel {
    panel_origin: Vec3,
    panel_normal: Vec3,
    material: Material,
}

impl Panel {
    pub fn new(panel_origin: Vec3, panel_normal: Vec3, material: Material) -> Self {
        return Self {
            panel_origin,
            panel_normal,
            material
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

        let t = ((self.panel_origin - (*ray.get_direction())).dot_product(&panel_normal)) / discriminant;

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
