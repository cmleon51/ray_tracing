use crate::canvas::RGB;
use crate::world::{Light, Object, ObjectRayIntersection, Ray, Vec3};

/// Object abstracting a point light in space
///
/// trough the use of this object we can simulate a "Point Light" by using the methods given by the
/// trait `Light`
pub struct PointLight {
    position: Vec3,
    intensity: f64,
    light_color: RGB,
}

impl PointLight {
    /// create a new point light
    pub fn new(position: Vec3, intensity: f64, light_color: Option<RGB>) -> Self {
        let light_color = match light_color {
            Some(light_color) => light_color,
            None => RGB::new(255, 255, 255)
        };
    
        return Self {
            position,
            intensity,
            light_color,
        };
    }
}

impl Light for PointLight {
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        other_objects: &Vec<Box<dyn Object>>,
        other_lights: &Vec<Box<dyn Light>>,
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
            let light_direction = self.position - point;
            let mut object_color = ray_object.get_hit_object().get_color(point);

            // after we get the light direction we need to compute if there are objects in our way
            // between the 'current_object' and the 'other_objects'
            let ray_to_light = Ray::new(point, light_direction);
            let light_length = light_direction.get_length();

            if let Some(hit_object) = ObjectRayIntersection::check_intersection(
                ray_to_light,
                other_objects,
                other_lights,
                0.001,
                light_length,
            ) {
                // if !hit_object.is_light_hit() {
                    return RGB::new(0, 0, 0);
                // }
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

            // using light_length to determine the light_intensity
            let attenuation = light_intensity / (1.0 + 0.09 * light_length + 0.032 * light_length * light_length);

            let mut final_color = (object_color) * (attenuation);

            // adding the light's color
            final_color = RGB::new(
                (f64::from(final_color.get_red()) * (f64::from(self.light_color.get_red()) / 255.0)) as u8,
                (f64::from(final_color.get_green()) * (f64::from(self.light_color.get_green()) / 255.0)) as u8,
                (f64::from(final_color.get_blue()) * (f64::from(self.light_color.get_blue()) / 255.0)) as u8,
            );

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
                            other_lights,
                            0.001,
                            f64::MAX,
                        ) {
                            if !hit_object.is_light_hit() {
                                refracted_color = self.compute_color(
                                    &hit_object,
                                    other_objects,
                                    other_lights,
                                    light_bounces - 1,
                                    background_color,
                                );
                            }else {
                                refracted_color = hit_object.get_hit_object().get_color(*hit_object.get_hit_point());
                            }
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

                    // we need to find the point and object that our 'ray_reflection' hits
                    let bounce_ray = Ray::new(point, ray_reflection);

                    if let Some(ray_object_intersection) = ObjectRayIntersection::check_intersection(
                        bounce_ray,
                        other_objects,
                        other_lights,
                        0.001,
                        f64::MAX,
                    ) {
                        if !ray_object_intersection.is_light_hit() {
                            reflected_color = self.compute_color(
                                &ray_object_intersection,
                                other_objects,
                                other_lights,
                                light_bounces - 1,
                                background_color,
                            );
                        }else {
                            reflected_color = ray_object_intersection.get_hit_object().get_color(*ray_object_intersection.get_hit_point());
                        }
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
