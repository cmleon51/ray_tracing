use crate::world::Vec3;
use crate::world::lights::Light;
use crate::world::objects::Object;

/// Object abstracting a directional light in space
///
/// trough the use of this object we can simulate a "Directional Light" by using the methods given by the
/// trait `Light`
pub struct DirectionalLight {
    direction: Vec3,
    intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, intensity: f64) -> Self {
        return Self {
            direction,
            intensity
        };
    }
}

impl Light for DirectionalLight {
    fn get_intensity(&self, point: Vec3, object: &Box<dyn Object>) -> Option<f64> {
        let object_normal = object.get_normal(point);

        return match object_normal {
            None => None,
            Some(normal) => {
                let mut light_intensity = 0.0;
                let light_direction = self.direction;
                let light_normal_dotproduct = normal.dot_product(&light_direction);

                if light_normal_dotproduct > 0.0 {
                    light_intensity = self.intensity * (light_normal_dotproduct / (normal.get_length() * light_direction.get_length()));
                }

                return Some(light_intensity);
            }
        }
    }
}
