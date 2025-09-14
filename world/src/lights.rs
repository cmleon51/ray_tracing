use crate::objects::Object;
use crate::{ObjectRayIntersection, Vec3};
use canvas::RGB;

/// trough this trait we can implement every type of light we may need for our ray traced world
pub trait Light {
    /// this function should return the current_object's color at the specified ray and t
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        other_objects: &[Box<dyn Object>],
        other_lights: &[Box<dyn Light>],
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB;

    /// this function returns the light's objects (necessary to implement area lights)
    fn get_object(&self) -> Option<&Box<dyn Object>> {
        None
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
    AmbientLight(f64),
    DirectionalLight(Vec3, f64),
    PanelLight(Vec3, f64, f64, Vec3, f64, f64, Option<RGB>),
    PointLight(Vec3, f64, Option<RGB>),
}

impl Lights {
    pub fn create_light(light: Lights) -> Box<dyn Light> {
        match light {
            Lights::AmbientLight(intensity) => Box::new(AmbientLight::new(intensity)),
            Lights::DirectionalLight(direction, intensity) => {
                Box::new(DirectionalLight::new(direction, intensity))
            }
            Lights::PanelLight(
                panel_origin,
                panel_width,
                panel_height,
                panel_normal,
                intensity,
                intersection_gap,
                light_color,
            ) => Box::new(PanelLight::new(
                panel_origin,
                panel_width,
                panel_height,
                panel_normal,
                intensity,
                intersection_gap,
                light_color,
            )),
            Lights::PointLight(position, intensity, light_color) => {
                Box::new(PointLight::new(position, intensity, light_color))
            }
        }
    }
}
