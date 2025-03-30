use crate::image::RGB;
use crate::world::Ray;
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
            intensity,
        };
    }
}

impl Light for PointLight {
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

        if let Some(normal) = current_object.get_normal(point) {
            let mut light_intensity = 0.0;
            let light_direction = self.position - point;

            // after we get the light direction we need to compute if there are objects in our way
            // between the 'current_object' and the 'other_objects'
            let mut smallest_t = f64::MAX;
            let light_length = light_direction.get_length();
            let ray_to_light = Ray::new(point, light_direction);
            for object in other_objects {
                let t = object.is_object_hit(&ray_to_light);

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

            if let Some(specularity) = *material.get_specularity() {
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

            let mut final_color = (*material.get_color()) * light_intensity;

            // even if we don't have light bounces we have to account for the object's
            // reflectiveness
            if let Some(reflection) = *material.get_reflectiveness() {
                let mut reflected_color = RGB::new(0, 0, 0);
                if light_bounces > 0 {
                    let ray_direction_inverted = (*ray.get_direction()) * -1.0;
                    let ray_reflection = ray_direction_inverted.reflect(&normal);

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
                        reflected_color = self.compute_color(
                            &bounce_ray,
                            smallest_t,
                            *bounce_ray.get_direction() * -1.0,
                            hit_object,
                            other_objects,
                            light_bounces - 1,
                        );
                    }
                }
                final_color = (final_color * (1.0 - reflection)) + (reflected_color * reflection);
            }

            return final_color;
        }

        return RGB::new(0, 0, 0);
    }
}
