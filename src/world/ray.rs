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
/// use ray_tracing_one_week::world::vec3::Vec3;
/// use ray_tracing_one_week::world::ray::Ray;
///
/// fn main() {
///     let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3(1.0, 0, 0));
/// }
///
/// ```
///
/// Calculate the ray's position at t
///
/// ```no_run
/// use ray_tracing_one_week::world::vec3::Vec3;
/// use ray_tracing_one_week::world::ray::Ray;
///
/// fn main() {
///     let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3(1.0, 0, 0));
///
///     let ray_position = ray.calculate_ray_position(0.8);
///
///     println!("{:?}", ray_position);
/// }
///
/// ```
#[derive(Debug)]
pub struct Ray {
    starting_position: Vec3,
    direction: Vec3,
}

impl Ray {
    /// creates a new instance of a ray
    pub fn new(starting_position: Vec3, direction: Vec3) -> Self {
        return Self {
            starting_position,
            direction,
        };
    }

    /// calculates the ray position at t
    pub fn calculate_ray_position(&self, t: f64) -> Vec3 {
        return self.starting_position + (self.direction * t);
    }

    /// retrieves a immutable reference to a ray's position
    pub fn get_position(&self) -> &Vec3 {
        return &self.starting_position;
    }

    /// retrieves a immutable reference to a ray's direction
    pub fn get_direction(&self) -> &Vec3 {
        return &self.direction;
    }
}
