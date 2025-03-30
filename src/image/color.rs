use std::ops;

/// An object to abstract a pixel's color
#[derive(Debug, Clone, Copy)]
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

    /// retrieves the rgb's red value
    pub fn get_red(&self) -> u8 {
        return self.red;
    }

    /// retrieves the rgb's green value
    pub fn get_green(&self) -> u8 {
        return self.green;
    }

    /// retrieves the rgb's blue value
    pub fn get_blue(&self) -> u8 {
        return self.blue;
    }

    /// sets the rgb's red value
    pub fn set_red(&mut self, new_red: u8) -> &mut Self {
        self.red = new_red;

        return self;
    }

    /// sets the rgb's green value
    pub fn set_green(&mut self, new_green: u8) -> &mut Self {
        self.green = new_green;

        return self;
    }

    /// sets the rgb's blue value
    pub fn set_blue(&mut self, new_blue: u8) -> &mut Self {
        self.blue = new_blue;

        return self;
    }
}

impl ops::Mul<f64> for RGB {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        return RGB::new(
            (f64::from(self.red) * rhs) as u8,
            (f64::from(self.green) * rhs) as u8,
            (f64::from(self.blue) * rhs) as u8,
        );
    }
}

impl ops::Add<RGB> for RGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return RGB::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        );
    }
}

impl ops::AddAssign<RGB> for RGB {
    fn add_assign(&mut self, rhs: RGB) {
        self.red = self.red.saturating_add(rhs.red);
        self.green = self.green.saturating_add(rhs.green);
        self.blue = self.blue.saturating_add(rhs.blue);
    }
}
