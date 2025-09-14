use crate::{Light, Object, ObjectRayIntersection, Ray, Vec3};
use canvas::RGB;

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
        Self {
            direction,
            intensity,
        }
    }
}

impl Light for DirectionalLight {
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        other_objects: &[Box<dyn Object>],
        other_lights: &[Box<dyn Light>],
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB {
        let point = *ray_object.get_hit_point();
        let material = ray_object.get_hit_object().get_material();
        let viewing_vector = ray_object.get_viewing_vector();
        let current_object = ray_object.get_hit_object();
        let ray_direction = ray_object.get_ray().get_direction();

        match current_object.get_normal(point) {
            Some(normal) => {
                let mut light_intensity = 0.0;
                let light_direction = self.direction;
                let object_color = ray_object.get_hit_object().get_color(point);

                // after we get the light direction we need to compute if there are objects in our way
                // between the 'current_object' and the 'other_objects'
                let ray_to_light = Ray::new(point, light_direction);
                let light_length = light_direction.get_length();

                if let Some(_hit_object) = ObjectRayIntersection::check_intersection(
                    ray_to_light,
                    other_objects,
                    &[],
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
                        let light_reflection_point_dot =
                            light_reflection.dot_product(viewing_vector);

                        if light_reflection_point_dot > 0.0 {
                            light_intensity += self.intensity
                                * (light_reflection_point_dot
                                    / (light_reflection.get_length()
                                        * viewing_vector.get_length()))
                                .powf(specularity);
                        }
                    }
                }

                let mut final_color = (object_color) * light_intensity;

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
                        let cos_theta =
                            f64::min((ray_direction.get_inverse()).dot_product(&normal), 1.0);
                        let r_out_perp =
                            ((*ray_direction) + (normal * cos_theta)) * refraction_index;
                        let r_out_parallel = normal
                            * (-(((1.0 - (r_out_perp.get_length() * r_out_perp.get_length()))
                                .abs())
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
                                other_lights,
                                0.001,
                                f64::MAX,
                            ) {
                                refracted_color = self.compute_color(
                                    &hit_object,
                                    other_objects,
                                    other_lights,
                                    light_bounces - 1,
                                    background_color,
                                );
                            } else {
                                refracted_color = background_color;
                            }
                        }

                        if let Some(transparency) = *material.get_transparency() {
                            final_color = final_color * (1.0 - transparency);
                        }

                        return refracted_color + final_color;
                    }
                }

                // even if we don't have light bounces we have to account for the object's
                // reflectiveness
                if let Some(reflection) = *material.get_reflectiveness() {
                    let mut reflected_color = RGB::new(0, 0, 0);
                    if light_bounces > 0 {
                        let ray_reflection = ray_direction.get_inverse().reflect(&normal);
                        let bounce_ray = Ray::new(point, ray_reflection);

                        // we need to find the point and object that our 'ray_reflection' hits
                        if let Some(ray_object_intersection) =
                            ObjectRayIntersection::check_intersection(
                                bounce_ray,
                                other_objects,
                                other_lights,
                                0.001,
                                f64::MAX,
                            )
                        {
                            reflected_color = self.compute_color(
                                &ray_object_intersection,
                                other_objects,
                                other_lights,
                                light_bounces - 1,
                                background_color,
                            );
                        } else {
                            reflected_color = background_color;
                        }
                    }

                    final_color =
                        (final_color * (1.0 - reflection)) + (reflected_color * reflection);
                }

                final_color
            }
            None => background_color,
        }
    }
}
