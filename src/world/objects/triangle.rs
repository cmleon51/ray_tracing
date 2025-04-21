use crate::canvas::RGB;
use crate::world::objects::{Material, Object};
use crate::world::{Ray, Vec3};

/// object to abstract a triangle in our ray traced world
///
/// Trough the use of this object we can create a triangle in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Triangle {
    vertice_1: Vec3,
    vertice_2: Vec3,
    vertice_3: Vec3,
    material: Material,
}

impl Triangle {
    pub fn new(vertice_1: Vec3, vertice_2: Vec3, vertice_3: Vec3, material: Material) -> Self {
        return Self {
            vertice_1,
            vertice_2,
            vertice_3,
            material,
        };
    }
}

impl Object for Triangle {
    fn is_object_hit(&self, ray: &Ray) -> Option<f64> {
        let e1 = self.vertice_2 - self.vertice_1;
        let e2 = self.vertice_3 - self.vertice_1;

        let ray_cross_e2 = ray.get_direction().cross_product(&e2);
        let det = e1.dot_product(&ray_cross_e2);

        if f64::abs(det) < f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = (*ray.get_position()) - self.vertice_1;
        let u = inv_det * s.dot_product(&ray_cross_e2);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross_product(&e1);
        let v = inv_det * ray.get_direction().dot_product(&s_cross_e1);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * e2.dot_product(&s_cross_e1);

        return Some(t);
    }

    fn get_normal(&self, point: Vec3) -> Option<Vec3> {
        let e1 = self.vertice_2 - self.vertice_1;
        let e2 = self.vertice_3 - self.vertice_1;

        return Some(e1.cross_product(&e2));
    }

    fn get_material(&self) -> &Material {
        return &(self.material);
    }

    fn get_color(&self, point: Vec3) -> RGB {
        return *self.get_material().get_color();
    }
}
