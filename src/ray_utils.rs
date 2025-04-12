use crate::canvas::*;
use crate::world::*;

/// function to retrieve a Vec allowing the user to store any object that implements `Hit`
pub fn create_objects_vec() -> Vec<Box<dyn Object>> {
    return vec![];
}

/// function to retrieve a Vec allowing the user to store any object that implements 'Light'
pub fn create_lights_vec() -> Vec<Box<dyn Light>> {
    return vec![];
}

/// function to retrieve the amount of increment on the x and y axis the viewport should move based
/// upon the given image, the increments will be in the positive x and positive y
pub fn get_viewport_xy_incr(camera: &Camera, image: &Canvas) -> (f64, f64) {
    let viewport_incr_x = (*camera.get_viewport_width()) / f64::from(image.get_width());
    let viewport_incr_y = (*camera.get_viewport_height()) / f64::from(image.get_height());

    return (viewport_incr_x, viewport_incr_y);
}
