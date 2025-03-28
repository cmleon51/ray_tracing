use crate::world::Vec3;
use crate::world::lights::Light;
use crate::world::objects::Object;

/// Object abstracting a point light in space
///
/// trough the use of this object we can simulate a "Point Light" by using the methods given by the
/// trait `Light`
pub struct PointLight {
    position: Vec3,
    intensity: f64,
}

impl PointLight {
    pub fn new(position: Vec3, intensity: f64) -> Self {
        return Self {
            position,
            intensity
        };
    }
}

impl Light for PointLight {
    fn get_intensity(&self, point: Vec3, viewing_vector: Vec3, object: &Box<dyn Object>) -> Option<f64> {
        if let Some(normal) = object.get_normal(point) {
            let mut light_intensity = 0.0;
            let light_direction = self.position - point;
            let light_normal_dotproduct = normal.dot_product(&light_direction);

            if light_normal_dotproduct > 0.0 {
                light_intensity += self.intensity * (light_normal_dotproduct / (normal.get_length() * light_direction.get_length()));
            }

            if let Some(specularity) = object.get_specularity() {
                if specularity >= 0.0 {    
                    // recuperiamo il vettore R (riflesso di 'light_direction')
                    // non c'è bisogno di fare il 'sqrt' di light_normal_dotproduct in quanto il
                    // normale è già normalizzato
                    let light_reflection = ((normal * 2.0) * light_normal_dotproduct) - light_direction;
                    let light_reflection_point_dot = light_reflection.dot_product(&viewing_vector);

                    if light_reflection_point_dot > 0.0 {
                        light_intensity += self.intensity * (light_reflection_point_dot / (light_reflection.get_length() * viewing_vector.get_length())).powf(specularity);
                    }
                }
            }

            return Some(light_intensity);
        }

        return None;
    }
}
