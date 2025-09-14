#![allow(clippy::too_many_arguments)]

use canvas::*;
use world::*;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    background_color: RGB, // until a skybox is implemented the "sky" will be a background color
    viewport_incr_x: f64,
    viewport_incr_y: f64,
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,
    pixel_samples: u32,
}

impl RayTracer {
    /// creates a new ray tracer
    pub fn new(
        camera_position: Vec3,
        camera_lookat: Vec3,
        up_vector: Vec3,
        canvas_width: u32,
        canvas_height: u32,
        background_color: RGB,
        viewport_width: f64,
        pixel_samples: u32,
    ) -> Self {
        let canvas = Canvas::new(canvas_width, canvas_height, background_color);
        let camera = Camera::new(
            camera_position,
            camera_lookat,
            up_vector,
            viewport_width,
            canvas.get_aspect_ratio(),
        );
        let viewport_incr_x = (*camera.get_viewport_width()) / f64::from(canvas.get_width());
        let viewport_incr_y = (*camera.get_viewport_height()) / f64::from(canvas.get_height());

        RayTracer {
            camera,
            canvas,
            background_color,
            viewport_incr_x,
            viewport_incr_y,
            objects: vec![],
            lights: vec![],
            pixel_samples,
        }
    }

    /// adds a new object in the ray tracer
    pub fn add_object(&mut self, new_object: Box<dyn Object>) {
        self.objects.push(new_object);
    }

    /// addsa new light in the ray tracer
    pub fn add_light(&mut self, new_light: Box<dyn Light>) {
        self.lights.push(new_light);
    }

    /// this functions traces a ray between the starting and end position, returning an RGB color
    pub fn trace_ray(&self, starting_position: Vec3, end_position: Vec3) -> RGB {
        let mut final_red: u32 = 0;
        let mut final_green: u32 = 0;
        let mut final_blue: u32 = 0;

        for _ in 0..self.pixel_samples {
            let mut ray = Ray::new(starting_position, end_position - starting_position);
            ray.scatter(
                Some(-self.viewport_incr_x..self.viewport_incr_x),
                Some(-self.viewport_incr_y..self.viewport_incr_y),
                None,
            );

            if let Some(object_intersection) = ObjectRayIntersection::check_intersection(
                ray,
                &self.objects,
                &self.lights,
                1.0,
                f64::MAX,
            ) {
                if !object_intersection.is_light_hit() {
                    for light in &self.lights {
                        let hit_color = light.compute_color(
                            &object_intersection,
                            &self.objects,
                            &self.lights,
                            3,
                            self.background_color,
                        );

                        final_red = final_red.saturating_add(u32::from(hit_color.get_red()));
                        final_green = final_green.saturating_add(u32::from(hit_color.get_green()));
                        final_blue = final_blue.saturating_add(u32::from(hit_color.get_blue()));
                    }
                } else {
                    // we return just the objects color
                    let object_color = object_intersection
                        .get_hit_object()
                        .get_color(*object_intersection.get_hit_point());
                    final_red = final_red.saturating_add(u32::from(object_color.get_red()));
                    final_green = final_green.saturating_add(u32::from(object_color.get_green()));
                    final_blue = final_blue.saturating_add(u32::from(object_color.get_blue()));
                }
            } else {
                final_red = final_red.saturating_add(u32::from(self.background_color.get_red()));
                final_green =
                    final_green.saturating_add(u32::from(self.background_color.get_green()));
                final_blue = final_blue.saturating_add(u32::from(self.background_color.get_blue()));
            }
        }

        RGB::new(
            final_red.saturating_div(self.pixel_samples).min(255) as u8,
            final_green.saturating_div(self.pixel_samples).min(255) as u8,
            final_blue.saturating_div(self.pixel_samples).min(255) as u8,
        )
    }

    /// this function renders the image on the "canvas"
    pub fn render(&mut self) {
        let mut canvas = std::mem::take(&mut self.canvas);
        let viewport_upper_left = self.camera.get_viewport_angle(ViewportAngles::UpperLeft);

        for pixel in &mut canvas {
            println!("{:?}", pixel);
            let pixel_center = viewport_upper_left
                + ((*self.camera.get_u_vector())
                    * (f64::from(pixel.get_x()) * self.viewport_incr_x))
                + ((*self.camera.get_v_vector())
                    * (f64::from(pixel.get_y()) * (self.viewport_incr_y * -1.0)));

            pixel.change_color(self.trace_ray(*self.camera.get_position(), pixel_center));
        }

        self.canvas = std::mem::take(&mut canvas);
    }

    /// this function returns the "canvas"
    pub fn get_canvas(&self) -> &Canvas {
        &self.canvas
    }
}
