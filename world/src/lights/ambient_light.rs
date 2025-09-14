use crate::{Light, Object, ObjectRayIntersection};
use canvas::RGB;

/// Object abstracting an ambient light
///
/// trough the use of this object we can simulate an "Ambient Light" by using the methods given by the
/// trait `Light`
pub struct AmbientLight {
    intensity: f64,
}

impl AmbientLight {
    pub fn new(intensity: f64) -> Self {
        Self { intensity }
    }
}

impl Light for AmbientLight {
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        _other_objects: &[Box<dyn Object>],
        _other_lights: &[Box<dyn Light>],
        _light_bounces: u8,
        _background_color: RGB,
    ) -> RGB {
        let point = *ray_object.get_hit_point();
        let object_color = ray_object.get_hit_object().get_color(point);

        match ray_object.get_hit_object().get_normal(point) {
            Some(_) => (object_color) * self.intensity,
            None => RGB::new(0, 0, 0),
        }
    }
}
