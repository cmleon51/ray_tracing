use crate::canvas::RGB;
use crate::world::{Light, Object, ObjectRayIntersection, Ray, Vec3};

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
        other_lights: &Vec<Box<dyn Light>>,
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB {
        let point = *ray_object.get_hit_point();
        let material = ray_object.get_hit_object().get_material();
        let object_color = ray_object.get_hit_object().get_color(point);

        if let Some(_) = ray_object.get_hit_object().get_normal(point) {
            return (object_color) * self.intensity;
        }

        return RGB::new(0, 0, 0);
    }
}
