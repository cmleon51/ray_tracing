use std::ops;

/// Object abstracting a vector in 3d space
///
/// Trough the use of this object we can calculate the cross/dot product between two vectors,
/// adding a vector to another, dividing a vector by another, multiplying a vector by a number and
/// everything else
///
/// # Examples
///
/// create a new Vec3 with all zero
///
///```no_run
/// use ray_tracing::world::Vec3;
///
/// fn main() {
///     let vector = Vec3::new(0.0, 0.0, 0.0);
/// }
///```
///
/// Execute addition between two vectors
///
///```no_run
/// use ray_tracing::world::Vec3;
///
/// fn main() {
///     let vector1 = Vec3::new(3.0, 1.0, 2.0);
///     let vector2 = Vec3::new(1.0, 2.0, 3.0);
///
///     let vector3 = vector1 + vector2;
///
///     println!("{:?}", vector3);
/// }
///```
///
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// creates a new instance of Vec3 with the given coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    /// executes the dot product between two Vec3
    pub fn dot_product(&self, other: &Self) -> f64 {
        let dot_x = self.x * other.x;
        let dot_y = self.y * other.y;
        let dot_z = self.z * other.z;

        return dot_x + dot_y + dot_z;
    }

    /// executes the cross product between two Vec3
    pub fn cross_product(&self, other: &Self) -> Self {
        let cross_x = (self.y * other.z) - (self.z * other.y);
        let cross_y = (self.z * other.x) - (self.x * other.z);
        let cross_z = (self.x * other.y) - (self.y * other.x);

        return Self::new(cross_x, cross_y, cross_z);
    }

    /// retrieves the x coordinate
    pub fn get_x(&self) -> &f64 {
        return &self.x;
    }

    /// retrieves the y coordinate
    pub fn get_y(&self) -> &f64 {
        return &self.y;
    }

    /// retrieves the z coordinate
    pub fn get_z(&self) -> &f64 {
        return &self.z;
    }

    /// sets the x coordinate
    pub fn set_x(&mut self, new_x: f64) -> &mut Self {
        self.x = new_x;

        return self;
    }

    /// sets the y coordinate
    pub fn set_y(&mut self, new_y: f64) -> &mut Self {
        self.y = new_y;

        return self;
    }

    /// sets the z coordinate
    pub fn set_z(&mut self, new_z: f64) -> &mut Self {
        self.z = new_z;

        return self;
    }

    /// retrieves the vector's magnitude (length)
    pub fn get_length(&self) -> f64 {
        return self.dot_product(self).sqrt();
    }

    /// makes the current vector a 'unit vector'
    pub fn make_unit(&mut self) -> &mut Self {
        let vector_length = self.get_length();
        (*self) = self.clone() / vector_length;

        return self;
    }

    /// calculates the reflected vector of the first parameter based upon the second one
    pub fn reflect(&self, respect: &Vec3) -> Vec3 {
        return (((*respect) * 2.0) * respect.dot_product(self)) - (*self);
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs);
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs);
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}
