use crate::canvas::RGB;
use crate::world::objects::{Material, Object};
use crate::world::{Ray, Vec3};

/// object to abstract a panel in our ray traced world
///
/// Trough the use of this object we can create a panel in our world and check if a ray hits it
/// with `is_object_hit`
pub struct Panel {
    panel_origin: Vec3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    material: Material,
}

impl Panel {
    pub fn new(
        panel_origin: Vec3,
        panel_width: f64,
        panel_height: f64,
        mut panel_normal: Vec3,
        material: Material,
    ) -> Self {
        panel_normal.make_unit();
        // these vectors are necessary to establish u and v
        let mut u = *panel_origin.cross_product(&panel_normal).make_unit();
        let mut v = panel_normal.cross_product(&u);

        // if the scalar_vectors are impossible then they are "normal"
        if u.get_x().is_nan() && u.get_y().is_nan() && u.get_z().is_nan()
            || v.get_x().is_nan() && v.get_y().is_nan() && v.get_z().is_nan()
        {
            u = Vec3::new(1.0, 0.0, 0.0);
            v = Vec3::new(0.0, 1.0, 0.0);
        }

        u = u * panel_width;
        v = v * panel_height;

        return Self {
            panel_origin,
            u,
            v,
            normal: panel_normal,
            material,
        };
    }
}

impl Object for Panel {
    fn is_object_hit(&self, ray: &Ray) -> Option<f64> {
        let u_v_cross = self.u.cross_product(&self.v);
        let discriminant = ray.get_direction().get_inverse().dot_product(&u_v_cross);

        if discriminant <= 0.001 && discriminant >= -0.001 {
            return None;
        }

        let t = u_v_cross.dot_product(&((*ray.get_position()) - self.panel_origin)) / discriminant;

        // u and v are calculated to check if the point lies inside or outside the plane
        let u_scalar = self
            .v
            .cross_product(&(ray.get_direction().get_inverse()))
            .dot_product(&((*ray.get_position()) - self.panel_origin))
            / ray.get_direction().get_inverse().dot_product(&u_v_cross);
        let v_scalar = ray
            .get_direction()
            .cross_product(&self.u)
            .dot_product(&((*ray.get_position()) - self.panel_origin))
            / ray.get_direction().get_inverse().dot_product(&u_v_cross);

        if u_scalar > 1.0 || u_scalar < -1.0 || v_scalar > 1.0 || v_scalar < -1.0 {
            return None;
        }

        return Some(t);
    }

    fn get_normal(&self, point: Vec3) -> Option<Vec3> {
        return Some(self.normal);
    }

    fn get_material(&self) -> &Material {
        return &(self.material);
    }

    fn get_color(&self, point: Vec3) -> RGB {
        return *self.get_material().get_color();
    }
}
