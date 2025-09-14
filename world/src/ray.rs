use std::ops::Range;

use super::vec3::Vec3;

/// An Object abstracting a Ray in 3d space
///
/// Trough the use of this object you can implement a ray moving trough 3d space
///
/// # Examples
///
/// Create a new ray at position (0, 0, 0) that is sent at direction (1, 0, 0)
///
/// ```no_run
/// let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
/// ```
///
/// Calculate the ray's position at t
///
/// ```no_run
/// let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
///
/// let ray_position = ray.calculate_ray_position(0.8);
///
/// println!("{:?}", ray_position);
///
/// ```
#[derive(Debug)]
pub struct Ray {
    starting_position: Vec3,
    direction: Vec3,
}

impl Ray {
    /// creates a new instance of a ray
    pub fn new(starting_position: Vec3, mut direction: Vec3) -> Self {
        direction.make_unit();

        Self {
            starting_position,
            direction,
        }
    }

    /// calculates the ray position at t
    pub fn calculate_ray_position(&self, t: f64) -> Vec3 {
        self.starting_position + (self.direction * t)
    }

    /// retrieves a immutable reference to a ray's position
    pub fn get_position(&self) -> &Vec3 {
        &self.starting_position
    }

    /// retrieves a immutable reference to a ray's direction
    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    /// scatters the ray's direction into the given x, y and z range
    pub fn scatter(
        &mut self,
        x_range: Option<Range<f64>>,
        y_range: Option<Range<f64>>,
        z_range: Option<Range<f64>>,
    ) -> &mut Self {
        let mut scatter_vector = Vec3::new(0.0, 0.0, 0.0);

        if let Some(range) = x_range {
            scatter_vector.add_x(rand::random_range(range));
        }

        if let Some(range) = y_range {
            scatter_vector.add_y(rand::random_range(range));
        }

        if let Some(range) = z_range {
            scatter_vector.add_z(rand::random_range(range));
        }

        self.direction += scatter_vector;

        self
    }
}
