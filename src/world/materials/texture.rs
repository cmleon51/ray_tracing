use crate::canvas::RGB;
use image::ImageReader;

/// An object rappresenting a texture
#[derive(Debug, Clone)]
pub struct Texture {
    image: Vec<RGB>,
    image_width: u32,
    image_height: u32,
}

impl Texture {
    pub fn load(image_path: &str) -> Self {
        let img = match ImageReader::open(image_path) {
            Ok(img) => match img.decode() {
                Ok(img) => img,
                Err(msg) => panic!("{:?}", msg),
            },
            Err(msg) => panic!("{:?}", msg),
        };
        let image_width = img.width();
        let image_height = img.height();
        let mut image_pixels: Vec<RGB> = vec![];

        for pixel in img.into_rgb8().pixels() {
            image_pixels.push(RGB::new(pixel[0], pixel[1], pixel[2]));
        }

        return Self {
            image_width,
            image_height,
            image: image_pixels,
        };
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> RGB {
        u = f64::floor(f64::clamp(u, 0.0, 1.0) * f64::from(self.image_width));
        v = f64::floor(f64::clamp(v, 0.0, 1.0) * f64::from(self.image_height));

        return self.image[(u + (v * f64::from(self.image_width))) as usize];
    }
}
