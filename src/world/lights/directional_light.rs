use crate::canvas::RGB;
use crate::world::lights::Light;
use crate::world::objects::{Material, Object};
use crate::world::{ObjectRayIntersection, Ray, Vec3};

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
        ray_object: &ObjectRayIntersection,
        other_objects: &Vec<Box<dyn Object>>,
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB {
        let point = *ray_object.get_hit_point();
        let material = ray_object.get_hit_object().get_material();
        let viewing_vector = ray_object.get_viewing_vector();
        let current_object = ray_object.get_hit_object();
        let ray_direction = ray_object.get_ray().get_direction();

        if let Some(normal) = current_object.get_normal(point) {
            let mut light_intensity = 0.0;
            let light_direction = self.direction;

            // after we get the light direction we need to compute if there are objects in our way
            // between the 'current_object' and the 'other_objects'
            let ray_to_light = Ray::new(point, light_direction);
            let light_length = light_direction.get_length();

            if let Some(hit_object) = ObjectRayIntersection::check_intersection(
                ray_to_light,
                other_objects,
                0.001,
                light_length,
            ) {
                return RGB::new(0, 0, 0);
            }

            // after we get the light direction we need to compute if there are objects in our way
            // between the 'current_object' and the 'other_objects'
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

            // calculate the refraction
            if let Some(material_refraction) = *material.get_refraction() {
                // check where the object is coming from (inside the material or outside of it)
                let ray_normal_dot = ray_direction.dot_product(&normal);

                let refraction_index = if ray_normal_dot < 0.0 {
                    // we are going into the material
                    1.0 / material_refraction
                } else {
                    material_refraction
                };

                // checking for "total internal reflection"
                let internal_reflection_res =
                    refraction_index * ray_direction.get_angle(&normal).sin();

                if internal_reflection_res < 1.0 {
                    let cos_theta = f64::min(((*ray_direction) * -1.0).dot_product(&normal), 1.0);
                    let r_out_perp = ((*ray_direction) + (normal * cos_theta)) * refraction_index;
                    let r_out_parallel = normal
                        * (-(((1.0 - (r_out_perp.get_length() * r_out_perp.get_length())).abs())
                            .sqrt()));

                    let refracted_direction = r_out_perp + r_out_parallel;

                    let refracted_ray = Ray::new(point, refracted_direction);

                    // check if the refracted ray hits anything, even if it doesn't we return that
                    // color
                    let mut refracted_color = RGB::new(0, 0, 0);

                    if light_bounces > 0 {
                        if let Some(hit_object) = ObjectRayIntersection::check_intersection(
                            refracted_ray,
                            other_objects,
                            0.001,
                            f64::MAX,
                        ) {
                            refracted_color = self.compute_color(
                                &hit_object,
                                other_objects,
                                light_bounces - 1,
                                background_color,
                            );
                        } else {
                            refracted_color = background_color;
                        }
                    }

                    return refracted_color;
                }
            }

            // even if we don't have light bounces we have to account for the object's
            // reflectiveness
            if let Some(reflection) = *material.get_reflectiveness() {
                let mut reflected_color = RGB::new(0, 0, 0);
                if light_bounces > 0 {
                    let ray_direction_inverted = (*ray_direction) * -1.0;
                    let ray_reflection = ray_direction_inverted.reflect(&normal);
                    let bounce_ray = Ray::new(point, ray_reflection);

                    // we need to find the point and object that our 'ray_reflection' hits
                    if let Some(ray_object_intersection) = ObjectRayIntersection::check_intersection(
                        bounce_ray,
                        other_objects,
                        0.001,
                        f64::MAX,
                    ) {
                        reflected_color = self.compute_color(
                            &ray_object_intersection,
                            other_objects,
                            light_bounces - 1,
                            background_color,
                        );
                    } else {
                        reflected_color = background_color;
                    }
                }

                final_color = (final_color * (1.0 - reflection)) + (reflected_color * reflection);
            }

            return final_color;
        }

        // nothing is hit
        return background_color;
    }
}
