use crate::canvas::RGB;
use crate::world::lights::Light;
use crate::world::objects::Object;
use crate::world::{ObjectRayIntersection, Ray, Vec3};

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
        ray_object: &ObjectRayIntersection,
        other_objects: &Vec<Box<dyn Object>>,
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB {
        let point = *ray_object.get_hit_point();
        let material = ray_object.get_hit_object().get_material();

        if let Some(_) = ray_object.get_hit_object().get_normal(point) {
            return (*material.get_color()) * self.intensity;
        }

        return RGB::new(0, 0, 0);
    }
}
