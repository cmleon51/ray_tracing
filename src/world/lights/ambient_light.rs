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
    fn get_intensity(&self, point: Vec3, object: &Box<dyn Object>) -> Option<f64> {
        let object_normal = object.get_normal(point);

        return match object_normal {
            None => None,
            Some(_) => Some(self.intensity),
        }
    }
}
