use crate::world::Vec3;
use crate::world::lights::Light;
use crate::world::objects::Object;

/// Object abstracting an ambient light
///
/// trough the use of this object we can simulate an "Ambient Light" by using the methods given by the
/// trait `Light`
pub struct AmbientLight {
    intensity: f64,
}

impl AmbientLight {
    pub fn new(intensity: f64) -> Self {
        return Self {
            intensity
        };
    }
}

impl Light for AmbientLight {
    fn get_intensity(&self, point: Vec3, viewing_vector: Vec3, current_object: &Box<dyn Object>, other_objects: &Vec<Box<dyn Object>>) -> Option<f64> {
        if let Some(_) = current_object.get_normal(point) {
            return Some(self.intensity);
        }

        return None;
    }
}
