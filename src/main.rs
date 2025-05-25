use ray_tracing::RGB;
use ray_tracing::RayTracer;
use ray_tracing::{
    AmbientLight, DirectionalLight, MaterialBuilder, Panel, PointLight, Sphere, Triangle, Vec3, PanelLight
};
use std::fs::File;
use std::io::prelude::Write;

fn main() {
    let mut ray_tracer = RayTracer::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        1200,
        1200,
        RGB::new(53, 81, 92),
        2.0,
        8,
    );

    // blue right sphere
    ray_tracer.add_object(Box::new(Sphere::new(
        Vec3::new(1.3, -3.0, 6.0),
        1.0,
        MaterialBuilder::new()
            .set_color(RGB::new(87, 87, 201))
            .set_reflectiveness(0.9)
            .build(),
    )));

    // yellow left sphere
    ray_tracer.add_object(Box::new(Sphere::new(
        Vec3::new(-1.0, -3.0, 5.0),
        1.0,
        MaterialBuilder::new()
            .set_color(RGB::new(183, 183, 78))
            .build(),
    )));

    // world sphere
    ray_tracer.add_object(Box::new(Sphere::new(
        Vec3::new(-2.0, 1.5, 5.0),
        0.8,
        MaterialBuilder::new()
            .set_texture("./textures/earthmap.jpg")
            .build(),
    )));

    // refracted sphere
    ray_tracer.add_object(Box::new(Sphere::new(
        Vec3::new(2.0, 1.5, 4.0),
        0.8,
        MaterialBuilder::new()
            .set_color(RGB::new(255, 255, 255))
            .set_refraction(1.55)
            .set_transparency(1.0)
            .build(),
    )));

    // Back panel
    ray_tracer.add_object(Box::new(Panel::new(
        Vec3::new(0.0, 0.0, 7.0),
        80.0,
        80.0,
        Vec3::new(0.0, 0.0, -1.0),
        MaterialBuilder::new()
            .set_color(RGB::new(233, 233, 233))
            .build(),
    )));

    // front panel
    ray_tracer.add_object(Box::new(Panel::new(
        Vec3::new(0.0, 0.0, 0.0),
        8.0,
        8.0,
        Vec3::new(0.0, 0.0, 1.0),
        MaterialBuilder::new()
            .set_color(RGB::new(233, 233, 233))
            .build(),
    )));

    // bottom panel
    ray_tracer.add_object(Box::new(Panel::new(
        Vec3::new(0.0, -4.0, 4.0),
        80.0,
        80.0,
        Vec3::new(0.0, 1.0, 0.0),
        MaterialBuilder::new()
            .set_color(RGB::new(233, 233, 233))
            .build(),
    )));

    // right panel
    ray_tracer.add_object(Box::new(Panel::new(
        Vec3::new(4.0, 0.0, 4.0),
        8.0,
        8.0,
        Vec3::new(-1.0, 0.0, 0.0),
        MaterialBuilder::new()
            .set_color(RGB::new(255, 118, 118))
            .build(),
    )));

    // left panel
    ray_tracer.add_object(Box::new(Panel::new(
        Vec3::new(-4.0, 0.0, 4.0),
        8.0,
        8.0,
        Vec3::new(1.0, 0.0, 0.0),
        MaterialBuilder::new()
            .set_color(RGB::new(100, 227, 106))
            .build(),
    )));

    // top panel
    ray_tracer.add_object(Box::new(Panel::new(
        Vec3::new(0.0, 4.0, 4.0),
        8.0,
        8.0,
        Vec3::new(0.0, -1.0, 0.0),
        MaterialBuilder::new()
            .set_color(RGB::new(233, 233, 233))
            .build(),
    )));

    ray_tracer.add_light(Box::new(PanelLight::new(
        Vec3::new(0.0, 2.5, 4.0),
        1.0,
        1.0,
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        0.05,
    )));

    ray_tracer.render();

    let mut file = match File::create("output.ppm") {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };

    let canvas = ray_tracer.get_canvas();

    let _ = file.write(b"P3\n");
    let _ = file.write(format!("{} {}\n", canvas.get_width(), canvas.get_height()).as_bytes());
    let _ = file.write(b"255\n");
    
    let gamma_correction_value = 1.0;

    for pixel in canvas {
        let pixel_color = pixel.get_color();

        // applying gamma correction
        let red = ((f64::from(pixel_color.get_red()) / 255.0).powf(gamma_correction_value) * 255.0) as u8;
        let green = ((f64::from(pixel_color.get_green()) / 255.0).powf(gamma_correction_value) * 255.0) as u8;
        let blue = ((f64::from(pixel_color.get_blue()) / 255.0).powf(gamma_correction_value) * 255.0) as u8;

        let _ = file.write(
            format!(
                "{} {} {}\n",
                red,
                green,
                blue,
            )
            .as_bytes(),
        );
    }
}
