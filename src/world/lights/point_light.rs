use crate::world::Vec3;
use crate::world::lights::Light;
use crate::world::objects::Object;
use crate::world::Ray;

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
    fn get_intensity(&self, point: Vec3, viewing_vector: Vec3, current_object: &Box<dyn Object>, other_objects: &Vec<Box<dyn Object>>) -> Option<f64> {
        if let Some(normal) = current_object.get_normal(point) {
            let mut light_intensity = 0.0;
            let light_direction = self.position - point;

            // after we get the light direction we need to compute if there are objects in our way
            // between the 'current_object' and the 'other_objects'
            let mut smallest_t = f64::MAX;
            for object in other_objects {
                let ray = Ray::new(point, light_direction);
                let t = object.is_object_hit(&ray);

                // we are checking if the t found for the object is before the light source and
                // more than the curernt position (have to use .get_length() since in our ray
                // implementation the direction is normalized)
                if t < light_direction.get_length() && t > 0.0 && t < smallest_t {
                    smallest_t = t;
                }
            }

            if smallest_t != f64::MAX {
                return None;
            }

            let light_normal_dotproduct = normal.dot_product(&light_direction);

            if light_normal_dotproduct > 0.0 {
                light_intensity += self.intensity * (light_normal_dotproduct / (normal.get_length() * light_direction.get_length()));
            }

            if let Some(specularity) = current_object.get_specularity() {
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
