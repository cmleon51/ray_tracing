/// An object to abstract a pixel's color
#[derive(Debug, Clone)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    /// creates a new RGB object
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RGB { red, green, blue }
    }

    pub fn red(&self) -> u8 {
        return self.red;
    }

    pub fn green(&self) -> u8 {
        return self.green;
    }

    pub fn blue(&self) -> u8 {
        return self.blue;
    }

    pub fn set_red(&mut self, new_red: u8) -> &mut Self {
        self.red = new_red;

        return self;
    }

    pub fn set_green(&mut self, new_green: u8) -> &mut Self {
        self.green = new_green;

        return self;
    }

    pub fn set_blue(&mut self, new_blue: u8) -> &mut Self {
        self.blue = new_blue;

        return self;
    }
}
