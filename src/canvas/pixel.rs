use super::color::RGB;

/// An object abstracting the idea of a pixel on an image
#[derive(Debug, Clone)]
pub struct Pixel {
    x: u32,
    y: u32,
    color: RGB,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: RGB) -> Self {
        return Self { x, y, color };
    }

    pub fn get_x(&self) -> u32 {
        return self.x;
    }

    pub fn get_y(&self) -> u32 {
        return self.y;
    }

    pub fn get_color(&self) -> &RGB {
        return &self.color;
    }

    pub fn change_color(&mut self, new_color: RGB) -> &mut Self {
        self.color = new_color;

        return self;
    }

    pub fn change_position(&mut self, new_x: Option<u32>, new_y: Option<u32>) -> &mut Self {
        if let Some(new_x) = new_x {
            self.x = new_x;
        }
        if let Some(new_y) = new_y {
            self.y = new_y;
        }

        return self;
    }
}
