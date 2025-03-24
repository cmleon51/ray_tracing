#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;

    const OUTPU_FOLDER: &str = "output/";

    #[test]
    /// this test creates a file of type "ppm" to test the image module
    fn create_ppm_image(){
        use ray_tracing::image::*;

        let file_path = format!("{}create_ppm_image.ppm", OUTPU_FOLDER);
        let image = Image::new(256, 256);

        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(error) => panic!("{}", error),
        };

        let _ = file.write(b"P3\n");
        let _ = file.write(format!("{} {}\n", image.get_width(), image.get_height()).as_bytes());
        let _ = file.write(b"255\n");

        let image_width = image.get_width();
        let image_height = image.get_width();

        for pixel in image.into_iter() {
            let red = f64::from(pixel.get_x()) / f64::from(image_width - 1);
            let green = f64::from(pixel.get_y()) / f64::from(image_height - 1);

            let _ = file.write(format!("{} {} {}\n", (255.0 * red).floor(), (255.0 * green).floor(), 0).as_bytes());
        }
    }

    #[test]
    /// this test creates a file of type "ppm" to test the "world" module
    /// current_version: 20250324
    fn create_world_scene() {
        use ray_tracing::image::Image;
        use ray_tracing::world::{Vec3, Ray, Camera, ViewportAngles};

        let mut image = Image::new(800, 400);
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.3, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            2.0,
            image.get_aspect_ratio(),
        );

        let viewport_upper_left = camera.get_viewport_angle(ViewportAngles::UpperLeft);
        let viewport_incr_x = (*camera.get_viewport_width()) / f64::from(image.get_width());
        let viewport_incr_y = ((*camera.get_viewport_height()) / f64::from(image.get_height())) * -1.0;

        let image_width = image.get_width();
        let image_height = image.get_height();

        let viewport_x_color = 255.0 / (*camera.get_viewport_width());
        let viewport_y_color = 255.0 / (*camera.get_viewport_height());

        let sphere_center = Vec3::new(0.0, 1.0, 2.0);
        let sphere_radius = 0.5;

        for pixel in &mut image {
            let pixel_location = viewport_upper_left + ((*camera.get_u_vector()) * (f64::from(pixel.get_x()) * viewport_incr_x)) + ((*camera.get_v_vector()) * (f64::from(pixel.get_y()) * viewport_incr_y));

            let ray = Ray::new(*camera.get_position(), pixel_location);

            let oc = (*ray.get_position()) - sphere_center;
            let a = ray.get_direction().dot_product(ray.get_direction());
            let b = ray.get_direction().dot_product(&oc) * 2.0;
            let c = oc.dot_product(&oc) - (sphere_radius * sphere_radius);
            let discriminant = b * b - 4.0 * a * c;

            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

            if t1 > 0.0 && t2 > 0.0 {
                pixel.change_color((((*pixel_location.get_x()) + (*camera.get_viewport_width()) / 2.0) * viewport_x_color) as u8, 0, (((*pixel_location.get_y()) + (*camera.get_viewport_height()) / 2.0) * viewport_y_color) as u8);
            }
        }

        let file_path = format!("{}create_world_scene.ppm", OUTPU_FOLDER);

        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(error) => panic!("{}", error),
        };

        let _ = file.write(b"P3\n");
        let _ = file.write(format!("{} {}\n", image_width, image_height).as_bytes());
        let _ = file.write(b"255\n");

        for pixel in &image {
            let pixel_color = pixel.get_color();
            let _ = file.write(format!("{} {} {}\n", pixel_color.red(), pixel_color.green(), pixel_color.blue()).as_bytes());
        }
    }
}
