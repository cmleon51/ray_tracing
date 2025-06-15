use crate::canvas::RGB;
use crate::ray_tracer::RayTracer;
use crate::world::objects::Object;
use crate::world::{ObjectRayIntersection, Ray, Vec3};

/// trough this trait we can implement every type of light we may need for our ray traced world
pub trait Light {
    /// this function should return the current_object's color at the specified ray and t
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        other_objects: &Vec<Box<dyn Object>>,
        other_lights: &Vec<Box<dyn Light>>,
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB;

    /// this function returns the light's objects (necessary to implement area lights)
    fn get_object(&self) -> Option<&Box<dyn Object>> {
        return None;
    }
}

/// module to implement an ambient light
mod ambient_light;
/// module to implement a directional light
mod directional_light;
/// moidule to implement a panel light
mod panel_light;
/// module to implement a point light
mod point_light;

// extracting everything we may need
use ambient_light::AmbientLight;
use directional_light::DirectionalLight;
use panel_light::PanelLight;
use point_light::PointLight;

/// enum containing all of the light's types we can create
pub enum Lights {
    AMBIENT_LIGHT(f64),
    DIRECTIONAL_LIGHT(Vec3, f64),
    PANEL_LIGHT(Vec3, f64, f64, Vec3, f64, f64, Option<RGB>),
    POINT_LIGHT(Vec3, f64, Option<RGB>),
}

impl Lights {
    pub fn create_light(light: Lights) -> Box<dyn Light> {
        match light {
            Lights::AMBIENT_LIGHT(intensity) => {
                return Box::new(AmbientLight::new(intensity));
            }
            Lights::DIRECTIONAL_LIGHT(direction, intensity) => {
                return Box::new(DirectionalLight::new(direction, intensity));
            }
            Lights::PANEL_LIGHT(
                panel_origin,
                panel_width,
                panel_height,
                panel_normal,
                mut intensity,
                intersection_gap,
                light_color,
            ) => {
                return Box::new(PanelLight::new(
                    panel_origin,
                    panel_width,
                    panel_height,
                    panel_normal,
                    intensity,
                    intersection_gap,
                    light_color,
                ));
            }
            Lights::POINT_LIGHT(position, intensity, light_color) => {
                return Box::new(PointLight::new(position, intensity, light_color));
            }
        };
    }
}
