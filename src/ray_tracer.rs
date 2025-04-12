use crate::canvas::*;
use crate::world::*;

pub struct RayTracer {
    camera: Camera,
    pub canvas: Canvas,
    background_color: RGB, // until a skybox is implemented the "sky" will be a background color
    viewport_incr_x: f64,
    viewport_incr_y: f64,
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,
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

        return RayTracer {
            camera,
            canvas,
            background_color,
            viewport_incr_x,
            viewport_incr_y,
            objects: vec![],
            lights: vec![],
        };
    }

    /// adds a new object in the ray tracer
    pub fn add_object(&mut self, new_object: Box<dyn Object>) -> &mut Self {
        self.objects.push(new_object);

        return self;
    }

    /// addsa new light in the ray tracer
    pub fn add_light(&mut self, new_light: Box<dyn Light>) -> &mut Self {
        self.lights.push(new_light);

        return self;
    }

    /// this functions traces a ray between the starting and end position, it returns an RGB color
    pub fn trace_ray(
        &self,
        starting_position: Vec3,
        end_position: Vec3,
    ) -> RGB {
        let mut hit_color = self.background_color;
        let ray = Ray::new(starting_position, end_position - starting_position);

        if let Some(object_intersection) = ObjectRayIntersection::check_intersection(ray, &self.objects, 1.0, f64::MAX) {
            hit_color = RGB::new(0, 0, 0);

            for light in &self.lights {
                hit_color += light.compute_color(&object_intersection, &self.objects, 3, self.background_color);
            }
        }

        return hit_color;
    }

    /// this function renders the image on the "canvas"
    pub fn render(&mut self) {
        let mut canvas = std::mem::take(&mut self.canvas);
        let viewport_upper_left = self.camera.get_viewport_angle(ViewportAngles::UpperLeft);

        for pixel in &mut canvas {
            let mut new_color: RGB = self.background_color;
            let pixel_center = viewport_upper_left
                + ((*self.camera.get_u_vector())
                    * (f64::from(pixel.get_x()) * self.viewport_incr_x))
                + ((*self.camera.get_v_vector())
                    * (f64::from(pixel.get_y()) * (self.viewport_incr_y * -1.0)));

            pixel.change_color(self.trace_ray(*self.camera.get_position(), pixel_center));
        }

        self.canvas = std::mem::take(&mut canvas);
    }

    // TODO: remove function
    // fn check_for_objects(&self, ray: Ray) -> Option<ObjectRayIntersection> {
    //     let mut smallest_t = f64::MAX;
    //     let mut hit_object: Option<&Box<dyn Object>> = None;
    //
    //     for object in &self.objects {
    //         let t = object.is_object_hit(&ray);
    //
    //         if t < smallest_t && t > 1.0 {
    //             smallest_t = t;
    //             hit_object = Some(object);
    //         }
    //     }
    //
    //     if let Some(hit_object) = hit_object {
    //         return Some(ObjectRayIntersection::new(ray, smallest_t, hit_object));
    //     }
    //
    //     return None;
    // }
}
