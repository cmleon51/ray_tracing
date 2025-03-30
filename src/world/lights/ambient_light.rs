use crate::image::RGB;
use crate::world::lights::Light;
use crate::world::objects::Object;
use crate::world::{Ray, Vec3};

/// Object abstracting an ambient light
///
/// trough the use of this object we can simulate an "Ambient Light" by using the methods given by the
/// trait `Light`
pub struct AmbientLight {
    intensity: f64,
}

impl AmbientLight {
    pub fn new(intensity: f64) -> Self {
        return Self { intensity };
    }
}

impl Light for AmbientLight {
    fn compute_color(
        &self,
        ray: &Ray,
        t: f64,
        viewing_vector: Vec3,
        current_object: &Box<dyn Object>,
        other_objects: &Vec<Box<dyn Object>>,
        light_bounces: u8,
    ) -> RGB {
        let point = ray.calculate_ray_position(t);
        let material = current_object.get_material();

        if let Some(_) = current_object.get_normal(point) {
            return (*material.get_color()) * self.intensity;
        }

        return RGB::new(0, 0, 0);
    }
}
