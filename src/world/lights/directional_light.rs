use crate::image::RGB;
use crate::world::lights::Light;
use crate::world::objects::Object;
use crate::world::{Ray, Vec3};

/// Object abstracting a directional light in space
///
/// trough the use of this object we can simulate a "Directional Light" by using the methods given by the
/// trait `Light`
pub struct DirectionalLight {
    direction: Vec3,
    intensity: f64,
}

impl DirectionalLight {
    /// creates a new directional light
    pub fn new(direction: Vec3, intensity: f64) -> Self {
        return Self {
            direction,
            intensity,
        };
    }
}

impl Light for DirectionalLight {
    fn compute_color(
        &self,
        ray: &Ray,
        t: f64,
        viewing_vector: Vec3,
        current_object: &Box<dyn Object>,
        other_objects: &Vec<Box<dyn Object>>,
        other_lights: &Vec<Box<dyn Light>>,
        light_bounces: u8,
    ) -> RGB {
        let point = ray.calculate_ray_position(t);

        if let Some(normal) = current_object.get_normal(point) {
            let mut light_intensity = 0.0;
            let light_direction = self.direction;

            // after we get the light direction we need to compute if there are objects in our way
            // between the 'current_object' and the 'other_objects'
            let mut smallest_t = f64::MAX;
            let light_length = light_direction.get_length();
            for object in other_objects {
                let ray = Ray::new(point, light_direction);
                let t = object.is_object_hit(&ray);

                // we are checking if the t found for the object is before the light source and
                // more than the curernt position (have to use .get_length() since in our ray
                // implementation the direction is normalized)
                if t < light_length && t > 0.001 && t < smallest_t {
                    smallest_t = t;
                }
            }

            if smallest_t != f64::MAX {
                return RGB::new(0, 0, 0);
            }

            let light_normal_dotproduct = normal.dot_product(&light_direction);

            if light_normal_dotproduct > 0.0 {
                light_intensity += self.intensity
                    * (light_normal_dotproduct
                        / (normal.get_length() * light_direction.get_length()));
            }

            if let Some(specularity) = current_object.get_specularity() {
                if specularity >= 0.0 {
                    let light_reflection = light_direction.reflect(&normal);
                    let light_reflection_point_dot = light_reflection.dot_product(&viewing_vector);

                    if light_reflection_point_dot > 0.0 {
                        light_intensity += self.intensity
                            * (light_reflection_point_dot
                                / (light_reflection.get_length() * viewing_vector.get_length()))
                            .powf(specularity);
                    }
                }
            }

            let mut final_color = (*current_object.get_color()) * light_intensity;

            if light_bounces > 0 {
                if let Some(reflection) = current_object.get_reflection() {
                    let ray_direction_inverted = (*ray.get_direction()) * -1.0;
                    let ray_reflection = ray_direction_inverted.reflect(&normal);

                    let mut reflected_color = RGB::new(0, 0, 0);

                    // we need to find the point and object that our 'ray_reflection' hits
                    let mut smallest_t = f64::MAX;
                    let mut hit_object = None;
                    let bounce_ray = Ray::new(point, ray_reflection);

                    for object in other_objects {
                        let t = object.is_object_hit(&bounce_ray);

                        if t > 0.001 && t < smallest_t {
                            smallest_t = t;
                            hit_object = Some(object);
                        }
                    }

                    if let Some(hit_object) = hit_object {
                        for light in other_lights {
                            reflected_color += light.compute_color(
                                &bounce_ray,
                                smallest_t,
                                *bounce_ray.get_direction() * -1.0,
                                hit_object,
                                other_objects,
                                other_lights,
                                light_bounces - 1,
                            );
                        }
                    }

                    final_color =
                        (final_color * (1.0 - reflection)) + (reflected_color * reflection);
                }
            }

            return final_color;
        }

        return RGB::new(0, 0, 0);
    }
}
