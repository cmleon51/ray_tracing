use super::vec3::Vec3;

/// An Object abstracting a Camera in a 3d world
///
/// Trough the use of this object we'll have all the necessary components to handle a camera in our
/// 3d world
///
/// # Examples
///
/// Create a new Camera
///
/// ```no_run
/// use ray_tracing_one_week::world::vec3::Vec3;
/// use ray_tracing_one_week::world::ray::Ray;
/// use ray_tracing_one_week::world::camera::Camera;
///
/// fn main {
///     let camera = Camera::new(
///         Vec3::new(0.0, 0.0, 0.0),
///         Vec3::new( 0.0, 0.0, 1.0),
///         Vec3::new( 0.0, 1.0, 0.0)
///     );
/// }
#[derive(Debug)]
pub struct Camera {
    position: Vec3,
    look_at: Vec3,
    up_vector: Vec3,
    u_vector: Vec3,
    v_vector: Vec3
}

impl Camera {
    /// create a new camera (the u_vector and v_vector are calculated automatically
    pub fn new(position: Vec3, look_at: Vec3, up_vector: Vec3) -> Self {
        let mut camera = Self {
            position,
            look_at,
            up_vector,
            u_vector: Vec3::new(0.0, 0.0, 0.0),
            v_vector: Vec3::new(0.0, 0.0, 0.0),
        };

        camera.update_camera();

        return camera;
    }

    /// updates the camera's position vector
    pub fn set_position(&mut self, new_position: Vec3) -> &mut Self {
        self.position = new_position;

        self.update_camera();

        return self;
    }

    /// updates the camera's look at vector
    pub fn set_look_at(&mut self, new_look_at: Vec3) -> &mut Self {
        self.look_at = new_look_at;

        self.update_camera();

        return self;
    }

    /// updates the camera's up vector
    pub fn set_up_vector(&mut self, new_up: Vec3) -> &mut Self {
        self.up_vector = new_up;

        self.update_camera();

        return self;
    }

    /// retrieves the camera's position vector
    pub fn get_position(&self) -> &Vec3 {
        return &self.position;
    }

    /// retrieves the camera's look at vector
    pub fn get_look_at(&self) -> &Vec3 {
        return &self.look_at;
    }

    /// retrieves the camera's up vector
    pub fn get_up_vector(&self) -> &Vec3 {
        return &self.up_vector;
    }

    /// retrieves the camera's u vector
    pub fn get_u_vector(&self) -> &Vec3 {
        return &self.u_vector;
    }

    /// retrieves the camera's v vector
    pub fn get_v_vector(&self) -> &Vec3 {
        return &self.v_vector;
    }

    /// updates the camera's u vector and v vector (this function is called every time the camera
    /// is updates)
    fn update_camera(&mut self) -> &mut Self {
        self.u_vector = self.up_vector.cross_product(&self.look_at);
        self.v_vector = self.u_vector.cross_product(&self.look_at);

        return self;
    }

}
