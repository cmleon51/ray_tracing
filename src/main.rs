use ray_tracing::image::{Image, RGB};
use ray_tracing::ray_utils;
use ray_tracing::world::{
    AmbientLight, Camera, DirectionalLight, PointLight, Ray, Sphere, Vec3, ViewportAngles,
};
use std::fs::File;
use std::io::prelude::Write;

fn main() {
    let mut image = Image::new(600, 600, RGB::new(0, 0, 0));
    let mut world_objects = ray_utils::create_objects_vec();
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        image.get_aspect_ratio(),
    );

    let viewport_upper_left = camera.get_viewport_angle(ViewportAngles::UpperLeft);
    let (viewport_incr_x, viewport_incr_y) = ray_utils::get_viewport_xy_incr(&camera, &image);

    let image_width = image.get_width();
    let image_height = image.get_height();

    world_objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1.0, 3.0),
        1.0,
        255,
        0,
        0,
        Some(500.0),
        Some(0.2),
    )));
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(2.0, 0.0, 4.0),
        1.0,
        0,
        0,
        255,
        Some(500.0),
        Some(0.3),
    )));
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(-2.0, 0.0, 4.0),
        1.0,
        0,
        255,
        0,
        Some(10.0),
        Some(0.4),
    )));
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -5001.0, 0.0),
        5000.0,
        255,
        255,
        0,
        Some(1000.0),
        Some(0.5),
    )));

    let mut world_lights = ray_utils::create_lights_vec();

    world_lights.push(Box::new(PointLight::new(Vec3::new(2.0, 1.0, 0.0), 0.6)));
    world_lights.push(Box::new(DirectionalLight::new(
        Vec3::new(1.0, 4.0, 4.0),
        0.2,
    )));
    world_lights.push(Box::new(AmbientLight::new(0.2)));

    for pixel in &mut image {
        let pixel_location = viewport_upper_left
            + ((*camera.get_u_vector()) * (f64::from(pixel.get_x()) * viewport_incr_x))
            + ((*camera.get_v_vector()) * (f64::from(pixel.get_y()) * (viewport_incr_y * -1.0)));

        let ray = Ray::new(
            *camera.get_position(),
            pixel_location - (*camera.get_position()),
        );

        let mut smallest_t = f64::MAX;
        for object in &world_objects {
            let t = object.is_object_hit(&ray);

            if t < smallest_t && t > 0.001 {
                smallest_t = t;
                let mut point_color = RGB::new(0, 0, 0);

                // light calculation
                for light in &world_lights {
                    point_color += light.compute_color(
                        &ray,
                        smallest_t,
                        (*ray.get_direction()) * -1.0,
                        object,
                        &world_objects,
                        &world_lights,
                        3,
                    );
                }

                pixel.change_color(point_color);
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
                pixel_color.get_red(),
                pixel_color.get_green(),
                pixel_color.get_blue()
            )
            .as_bytes(),
        );
    }
}
