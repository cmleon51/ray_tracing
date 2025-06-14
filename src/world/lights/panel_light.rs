use crate::canvas::RGB;
use crate::world::{
    Light, Lights, MaterialBuilder, Object, ObjectRayIntersection, Objects, Ray, Vec3,
};
use rand::Rng;

/// Object abstracting a panel area light
///
/// trough the use of this object we can simulate a "Panel Light" by using the methods given by the
/// trait `Light`
pub struct PanelLight {
    panel: Box<dyn Object>,
    intersection_points: Vec<Vec3>,
    intensity: f64,
}

impl PanelLight {
    pub fn new(
        panel_origin: Vec3,
        panel_width: f64,
        panel_height: f64,
        panel_normal: Vec3,
        mut intensity: f64,
        intersection_gap: f64,
        light_color: Option<RGB>,
    ) -> Self {
        let light_color = match light_color {
            Some(light_color) => light_color,
            None => RGB::new(255, 255, 255),
        };

        // we calculate how many steps we have to take based upon the 'intesection_gap' variable
        let width_steps: i32 = (panel_width / intersection_gap).floor() as i32;
        let height_steps: i32 = (panel_height / intersection_gap).floor() as i32;

        let panel_area = panel_width * panel_height;
        intensity /= panel_area;

        // multiplying the light_color by the intensity to give the user some feedback
        let panel = Objects::create_object(Objects::PANEL(
            panel_origin,
            panel_width,
            panel_height,
            panel_normal,
            MaterialBuilder::new()
                .set_color(light_color * (intensity.min(1.0)))
                .build(),
        ));

        // TODO: remove this code duplication
        // we calculate every point on the surface we have to check when doing light calculations
        let mut panel_u = *panel_origin.cross_product(&panel_normal).make_unit();
        let mut panel_v = panel_normal.cross_product(&panel_u);

        // if the scalar_vectors are impossible then they are "normal"
        if panel_u.get_x().is_nan() && panel_u.get_y().is_nan() && panel_u.get_z().is_nan()
            || panel_v.get_x().is_nan() && panel_v.get_y().is_nan() && panel_v.get_z().is_nan()
        {
            panel_u = Vec3::new(1.0, 0.0, 0.0);
            panel_v = Vec3::new(0.0, 1.0, 0.0);
        }

        panel_u = panel_u * (panel_width / 2.0);
        panel_v = panel_v * (panel_height / 2.0);

        // we start marking points on the top left of the panel
        let mut intersection_points: Vec<Vec3> = vec![];
        let panel_top_left = (panel_v - panel_u) + panel_origin;

        for i in 1..(width_steps - 1) {
            let i_step = (intersection_gap * f64::from(i)) / (panel_width / 2.0);
            let u_step = panel_u * i_step;

            for j in 1..(height_steps - 1) {
                let j_step = (intersection_gap * f64::from(j)) / (panel_height / 2.0);
                let v_step = panel_v.get_inverse() * j_step;

                // to check if an object is behind the light source i move the intersection_point a
                // little bit forward based upon the objects normal
                intersection_points.push(panel_top_left + u_step + v_step + (panel_normal * 0.001));
            }
        }

        return Self {
            panel: panel,
            intersection_points,
            intensity,
        };
    }
}

impl Light for PanelLight {
    fn compute_color(
        &self,
        ray_object: &ObjectRayIntersection,
        other_objects: &Vec<Box<dyn Object>>,
        other_lights: &Vec<Box<dyn Light>>,
        light_bounces: u8,
        background_color: RGB,
    ) -> RGB {
        // for every point on our panel light we create a special point light at that point, then
        // we average the result and we will get the correct light (i think)
        let point = *ray_object.get_hit_point();
        let current_object = ray_object.get_hit_object();

        if let Some(normal) = current_object.get_normal(point) {
            let mut final_red: u32 = 0;
            let mut final_green: u32 = 0;
            let mut final_blue: u32 = 0;
            for intersection_point in &self.intersection_points {
                let point_light = Lights::create_light(Lights::POINT_LIGHT(
                    *intersection_point,
                    self.intensity,
                    Some(*self.panel.get_material().get_color()),
                ));
                let calculated_color = point_light.compute_color(
                    ray_object,
                    other_objects,
                    other_lights,
                    light_bounces,
                    background_color,
                );

                final_red = final_red.saturating_add(u32::from(calculated_color.get_red()));
                final_green = final_green.saturating_add(u32::from(calculated_color.get_green()));
                final_blue = final_blue.saturating_add(u32::from(calculated_color.get_blue()));
            }

            let intersection_samples_count: u32 = self.intersection_points.len() as u32;
            let final_color = RGB::new(
                final_red
                    .saturating_div(intersection_samples_count)
                    .min(255) as u8,
                final_green
                    .saturating_div(intersection_samples_count)
                    .min(255) as u8,
                final_blue
                    .saturating_div(intersection_samples_count)
                    .min(255) as u8,
            );

            return final_color;
        }

        return RGB::new(0, 0, 0);
    }

    fn get_object(&self) -> Option<&Box<dyn Object>> {
        return Some(&self.panel);
    }
}
