use super::color::RGB;

#[derive(Debug)]
pub struct Pixel {
    x: u32,
    y: u32,
    color: RGB,
}

impl Pixel {
    pub fn new(x: u32, y: u32, red: u8, green: u8, blue: u8) -> Self {
        Self {
            x,
            y,
            color: RGB::new(red, green, blue),
        }
    }

    pub fn get_x(&self) -> &u32 {
        &self.x
    }

    pub fn get_y(&self) -> &u32 {
        &self.y
    }

    pub fn get_color(&self) -> &RGB {
        &self.color
    }

    pub fn change_color(&mut self, new_color: RGB) {
        self.color = new_color;
    }
}
