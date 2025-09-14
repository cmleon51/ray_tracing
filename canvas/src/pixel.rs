use crate::color::RGB;

/// An object abstracting the idea of a pixel on an image
#[derive(Debug, Clone)]
pub struct Pixel {
    x: u32,
    y: u32,
    color: RGB,
}

impl Pixel {
    /// Create a new pixel with a specific position and color
    pub fn new(x: u32, y: u32, color: RGB) -> Self {
        Self { x, y, color }
    }

    /// retrieve the x position
    pub fn get_x(&self) -> u32 {
        self.x
    }

    /// retrieve the y position
    pub fn get_y(&self) -> u32 {
        self.y
    }

    /// retrieve the pixel's color
    pub fn get_color(&self) -> &RGB {
        &self.color
    }

    /// change the pixel's color
    pub fn change_color(&mut self, new_color: RGB) -> &mut Self {
        self.color = new_color;

        self
    }
}
