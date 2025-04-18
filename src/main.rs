use ray_tracing::RGB;
use ray_tracing::RayTracer;
use ray_tracing::{
    AmbientLight, DirectionalLight, Material, PointLight, Sphere, Triangle, Vec3,
};
use std::fs::File;
use std::io::prelude::Write;

fn main() {
    let mut ray_tracer = RayTracer::new(
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        600,
        600,
        RGB::new(0, 0, 0),
        2.0,
    );

    ray_tracer.add_object(Box::new(Triangle::new(
        Vec3::new(0.0, 5.0, 5.0),
        Vec3::new(5.0, -5.0, 5.0),
        Vec3::new(-5.0, -5.0, 5.0),
        Material::new(RGB::new(255, 255, 255), Some(0.7), None, None),
    )));
    // ray_tracer.add_object(Box::new(Sphere::new(
    //     Vec3::new(0.0, -5001.0, 0.0),
    //     5000.0,
    //     Material::new(RGB::new(255, 255, 0), None, None, None),
    // )));
    ray_tracer.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 3.0),
        1.0,
        Material::new(RGB::new(255, 0, 0), None, Some(50.0), None),
    )));
    // ray_tracer.add_object(Box::new(Sphere::new(
    //     Vec3::new(2.0, 0.0, 4.0),
    //     1.0,
    //     Material::new(RGB::new(204, 204, 204), Some(1.0), None, Some(1.50)),
    // )));
    // ray_tracer.add_object(Box::new(Sphere::new(
    //     Vec3::new(-2.0, 0.0, 4.0),
    //     1.0,
    //     Material::new(RGB::new(0, 255, 0), Some(1.0), None, None),
    // )));
    //
    ray_tracer.add_light(Box::new(PointLight::new(Vec3::new(2.0, 1.0, 0.0), 0.6)));
    ray_tracer.add_light(Box::new(DirectionalLight::new(
        Vec3::new(2.0, 1.0, 0.0),
        0.4,
    )));

    ray_tracer.render();

    let mut file = match File::create("output.ppm") {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };

    let canvas = ray_tracer.get_canvas();

    let _ = file.write(b"P3\n");
    let _ = file.write(
        format!(
            "{} {}\n",
            canvas.get_width(),
            canvas.get_height()
        )
        .as_bytes(),
    );
    let _ = file.write(b"255\n");

    for pixel in canvas {
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
