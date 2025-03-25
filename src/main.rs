use std::fs::File;
use std::io::prelude::Write;
use ray_tracing::image::Image;
use ray_tracing::world::{self, Camera, Ray, Vec3, ViewportAngles};

fn main() {
    let mut image = Image::new(800, 400);
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        2.0,
        image.get_aspect_ratio(),
    );

    let viewport_upper_left = camera.get_viewport_angle(ViewportAngles::UpperLeft);
    let viewport_incr_x = (*camera.get_viewport_width()) / f64::from(image.get_width());
    let viewport_incr_y =
        ((*camera.get_viewport_height()) / f64::from(image.get_height())) * -1.0;

    let image_width = image.get_width();
    let image_height = image.get_height();

    let viewport_x_color = 255.0 / (*camera.get_viewport_width());
    let viewport_y_color = 255.0 / (*camera.get_viewport_height());

    let mut world_objects = world::create_objects_vec();

    world_objects.push(Box::new(world::Sphere::new(Vec3::new(0.0, 0.0, 2.0), 0.4)));
    world_objects.push(Box::new(world::Sphere::new(Vec3::new(1.0, 0.0, 2.0), 0.4)));

    for pixel in &mut image {
        let pixel_location = viewport_upper_left
            + ((*camera.get_u_vector()) * (f64::from(pixel.get_x()) * viewport_incr_x))
            + ((*camera.get_v_vector()) * (f64::from(pixel.get_y()) * viewport_incr_y));

        let ray = Ray::new(*camera.get_position(), pixel_location);

        for object in &world_objects {
            let t = object.is_object_hit(&ray);

            if t > 0.0 {
                pixel.change_color((((*pixel_location.get_x()) + (*camera.get_viewport_width()) / 2.0) * viewport_x_color) as u8, 0, (((*pixel_location.get_y()) + (*camera.get_viewport_height()) / 2.0) * viewport_y_color) as u8);
            }
        }
    }

    let mut file = match File::create("output.ppm") {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };

    let _ = file.write(b"P3\n");
    let _ = file.write(format!("{} {}\n", image_width, image_height).as_bytes());
    let _ = file.write(b"255\n");

    for pixel in &image {
        let pixel_color = pixel.get_color();
        let _ = file.write(
            format!(
                "{} {} {}\n",
                pixel_color.red(),
                pixel_color.green(),
                pixel_color.blue()
            )
            .as_bytes(),
        );
    }
}
