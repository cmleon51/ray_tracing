mod color;
mod pixel;

use pixel::Pixel;

/// an object abstracting the output Image
///
/// An instance of a _Image_ can be used to create the necessary output of our ray traced world by
/// implementing the trait `Render`
///
/// # Examples
///
/// Create a new image of size 256 x 256:
///
/// ```no_run
/// use ray_tracing::image::Image;
///
/// fn main() {
///     let image = Image::new(256, 256);
/// }
/// ```
///
/// Iterates over all of the [`Pixels`] with immutable references
///
/// ```no_run
/// use ray_tracing::image::Image;
///
/// fn main() {
///     let image = Image::new(256, 256);
///
///     for pixel in image.into_iter() {
///         println!("{:?}", pixel);
///     }
/// }
/// ```
///
/// Iterates over all of the [`Pixels`] with mutable references
///
/// ```no_run
/// use ray_tracing::image::Image;
///
/// fn main() {
///     let mut image = Image::new(256, 256);
///
///     for pixel in (&mut image).into_iter() {
///         println!("{:?}", pixel);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    aspect_ratio: f64,
}

impl Image {
    /// Creates a new image based upon the given `width` and `height` and creates all of the
    /// [`Pixels`] colored white
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels: Vec<Pixel> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                pixels.push(Pixel::new(x, y, 255, 255, 255));
            }
        }

        return Self {
            width,
            height,
            pixels,
            aspect_ratio: f64::from(width) / f64::from(height),
        };
    }

    /// returns a copy of the `width` of the image
    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    /// returns a copy of the `height` of the image
    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    /// returns a copy of the `aspect ratio` of the image
    pub fn get_aspect_ratio(&self) -> f64 {
        return self.aspect_ratio;
    }
}

/// trait for implementing a `rendering function` for the given Image
pub trait Render {
    /// renders and consumes the given Image and returns a unit value if everything went well and a string slice
    /// if something went wrong trough the use of `Result`
    fn render_image(&self, image: Image) -> Result<(), &str>;
}

impl<'a> IntoIterator for &'a Image {
    type Item = &'a Pixel;
    type IntoIter = ImageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator {
            pixels: &self.pixels,
            index: 0,
        }
    }
}

/// Struct for executing the `IntoIterator` Trait upon a `&Image` type
pub struct ImageIterator<'a> {
    pixels: &'a Vec<Pixel>,
    index: usize,
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = &'a Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pixels.get(self.index);
        self.index += 1;

        return pixel;
    }
}

// TODO: understand how this iterator over mutable references works
impl<'a> IntoIterator for &'a mut Image {
    type Item = &'a mut Pixel;
    type IntoIter = ImageMutIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ImageMutIterator {
            pixels: self.pixels.iter_mut(),
        }
    }
}

/// Struct for executing the `IntoIterator` Trait upon a `&mut Image` type
pub struct ImageMutIterator<'a> {
    pixels: std::slice::IterMut<'a, Pixel>,
}

impl<'a> Iterator for ImageMutIterator<'a> {
    type Item = &'a mut Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        self.pixels.next()
    }
}
