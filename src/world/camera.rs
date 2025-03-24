use super::vec3::Vec3;

/// An Object abstracting the viewport 
///
/// This object is not public since i wanted to make the viewport always stay at the camera's
/// look_at vector
#[derive(Debug)]
struct Viewport {
    pub width: f64,
    pub height: f64,
    pub position: Vec3
}

/// An Enum defining each and every angle of the viewport
pub enum ViewportAngles {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight
}

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
/// use ray_tracing::world::Vec3;
/// use ray_tracing::world::Ray;
/// use ray_tracing::world::Camera;
///
/// fn main() {
///     let camera = Camera::new(
///         Vec3::new(0.0, 0.0, 0.0),
///         Vec3::new( 0.0, 0.0, 1.0),
///         Vec3::new( 0.0, 1.0, 0.0),
///         2.0,
///         16.0 / 9.0
///     );
/// }
#[derive(Debug)]
pub struct Camera {
    position: Vec3,
    look_at: Vec3,
    up_vector: Vec3,
    u_vector: Vec3,
    v_vector: Vec3,
    viewport: Viewport,
}

impl Camera {
    /// create a new camera (the u_vector and v_vector are calculated automatically
    pub fn new(position: Vec3, mut look_at: Vec3, up_vector: Vec3, viewport_width: f64, aspect_ratio: f64) -> Self {
        look_at.make_unit();

        let mut camera = Self {
            position,
            look_at,
            up_vector,
            u_vector: Vec3::new(0.0, 0.0, 0.0),
            v_vector: Vec3::new(0.0, 0.0, 0.0),
            viewport: Viewport {
                width: viewport_width,
                height: viewport_width / aspect_ratio,
                position: look_at
            }
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
    pub fn set_look_at(&mut self, mut new_look_at: Vec3) -> &mut Self {
        self.look_at = *new_look_at.make_unit();

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

    /// retrieves the camera's viewport width
    pub fn get_viewport_width(&self) -> &f64 {
        return &self.viewport.width;
    }

    /// retrieves the camera's viewport height
    pub fn get_viewport_height(&self) -> &f64 {
        return &self.viewport.height;
    }

    /// retrieves the specified ['ViewportAngle'] of the viewport
    pub fn get_viewport_angle(&self, wich_angle: ViewportAngles) -> Vec3 {
         return match wich_angle {
            ViewportAngles::UpperLeft => self.position - (self.u_vector * (self.viewport.width / 2.0)) + (self.v_vector
                * self.viewport.height / 2.0) + self.look_at,
            ViewportAngles::UpperRight => self.position + (self.u_vector * (self.viewport.width / 2.0)) + (self.v_vector
                * self.viewport.height / 2.0) + self.look_at,
            ViewportAngles::LowerLeft => self.position - (self.u_vector * (self.viewport.width / 2.0)) - (self.v_vector
                * self.viewport.height / 2.0) + self.look_at,
            ViewportAngles::LowerRight => self.position + (self.u_vector * (self.viewport.width / 2.0)) - (self.v_vector
                * self.viewport.height / 2.0) + self.look_at,
        }
    }

    /// updates the camera's u vector, v vector and viewport (this function is called every time the camera
    /// is updates)
    fn update_camera(&mut self) -> &mut Self {
        self.u_vector = self.up_vector.cross_product(&self.look_at);
        self.v_vector = self.look_at.cross_product(&self.u_vector);
        self.viewport.position = self.look_at;

        self.u_vector.make_unit();
        self.v_vector.make_unit();
        self.look_at.make_unit();

        return self;
    }

}
