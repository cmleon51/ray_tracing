pub use crate::color::RGB;
pub use crate::pixel::Pixel;

#[derive(Debug, Default)]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    aspect_ratio: f64,
}

impl Canvas {
    /// Creates a new image based upon the given `width` and `height` and creates all of the
    /// [`Pixels`] colored white
    pub fn new(width: u32, height: u32, background_color: RGB) -> Self {
        let mut pixels: Vec<Pixel> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                pixels.push(Pixel::new(x, y, background_color));
            }
        }

        Self {
            width,
            height,
            pixels,
            aspect_ratio: f64::from(width) / f64::from(height),
        }
    }

    /// returns a copy of the `width` of the image
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// returns a copy of the `height` of the image
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// returns a copy of the `aspect ratio` of the image
    pub fn get_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
}

impl<'a> IntoIterator for &'a Canvas {
    type Item = &'a Pixel;
    type IntoIter = CanvasIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CanvasIterator {
            pixels: &self.pixels,
            index: 0,
        }
    }
}

/// Struct for executing the `IntoIterator` Trait upon a `&Canvas` type
pub struct CanvasIterator<'a> {
    pixels: &'a Vec<Pixel>,
    index: usize,
}

impl<'a> Iterator for CanvasIterator<'a> {
    type Item = &'a Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pixels.get(self.index);
        self.index += 1;

        pixel
    }
}

// TODO: understand how this iterator over mutable references works
impl<'a> IntoIterator for &'a mut Canvas {
    type Item = &'a mut Pixel;
    type IntoIter = CanvasMutIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CanvasMutIterator {
            pixels: self.pixels.iter_mut(),
        }
    }
}

/// Struct for executing the `IntoIterator` Trait upon a `&mut Canvas` type
pub struct CanvasMutIterator<'a> {
    pixels: std::slice::IterMut<'a, Pixel>,
}

impl<'a> Iterator for CanvasMutIterator<'a> {
    type Item = &'a mut Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        self.pixels.next()
    }
}
