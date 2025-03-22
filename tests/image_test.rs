use ray_tracing_one_week::image::*;
use std::fs::File;
use std::io::prelude::*;

#[test]
/// this test creates a file of type "ppm" to test the image crate
fn create_ppm_image(){
    const TEST_FOLDER: &str = "tests/";

    let file_path = format!("{}image_test.ppm", TEST_FOLDER);
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
