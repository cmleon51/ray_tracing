use crate::canvas::RGB;
use crate::world::Light;
use crate::world::Material;
use crate::world::Ray;
use crate::world::Vec3;

/// An object rappresenting the intersection between an object and a ray
pub struct ObjectRayIntersection<'a> {
    ray: Ray,
    t: f64,
    viewing_vector: Vec3,
    object: &'a Box<dyn Object>,
    object_point: Vec3,
    is_light_hit: bool,
}

impl<'a> ObjectRayIntersection<'a> {
    /// creates a new ObjectRayIntersection object, it's a private function since only
    /// `check_intersection` should be really used
    fn new(ray: Ray, t: f64, object: &'a Box<dyn Object>, is_light_hit: bool) -> Self {
        let object_point = ray.calculate_ray_position(t);
        let viewing_vector = ray.get_direction().get_inverse();

        return Self {
            ray,
            t,
            viewing_vector,
            object,
            object_point,
            is_light_hit,
        };
    }

    /// this function checks for an interaction between the given ray and the list of world objects
    pub fn check_intersection(
        ray: Ray,
        objects: &'a Vec<Box<dyn Object>>,
        lights: &'a Vec<Box<dyn Light>>,
        min_t: f64,
        max_t: f64,
    ) -> Option<Self> {
        let mut smallest_t = f64::MAX;
        let mut hit_object: Option<&Box<dyn Object>> = None;
        let mut is_light_hit = false;

        for object in objects {
            match object.is_object_hit(&ray) {
                Some(t) => {
                    if t < smallest_t && t > min_t && t < max_t {
                        smallest_t = t;
                        hit_object = Some(object);
                    }
                }
                None => (),
            }
        }

        // i check if a light is being hit before the object
        for light in lights {
            if let Some(light_object) = light.get_object() {
                match light_object.is_object_hit(&ray) {
                    Some(t) => {
                        if t < smallest_t && t > min_t && t < max_t {
                            smallest_t = t;
                            is_light_hit = true;
                            hit_object = Some(light_object);
                        }
                    }
                    None => (),
                }
            }
        }

        if let Some(hit_object) = hit_object {
            return Some(Self::new(ray, smallest_t, hit_object, is_light_hit));
        }

        return None;
    }

    /// returns the point at which the object has been hit
    pub fn get_hit_point(&self) -> &Vec3 {
        return &self.object_point;
    }

    /// returns the hit object
    pub fn get_hit_object(&self) -> &Box<dyn Object> {
        return self.object;
    }

    /// returns the vector that points from the `hit point` to the ray's starting position
    pub fn get_viewing_vector(&self) -> &Vec3 {
        return &self.viewing_vector;
    }

    /// retrieves the ray
    pub fn get_ray(&self) -> &Ray {
        return &self.ray;
    }

    /// tells the user if it's a light that is being hit
    pub fn is_light_hit(&self) -> bool {
        return self.is_light_hit;
    }
}

/// trait for implementing the necessary functions to make a type a `Object`
pub trait Object {
    /// this method should do all of the necessary calculations to check if a ray hits an object
    /// and return the `t` ray parameter that is closest to the ray
    fn is_object_hit(&self, ray: &Ray) -> Option<f64>;

    /// this method should retun a unit vector of the normal of the object based upon a point in
    /// space that is on the surface of the object (the value could be None if the point given is
    /// not on the surface
    fn get_normal(&self, point: Vec3) -> Option<Vec3>;

    /// this methos should return the object's material
    fn get_material(&self) -> &Material;

    /// this method should return the color of the sphere at the specified point
    fn get_color(&self, point: Vec3) -> RGB;
}

mod panel;
/// modules implementing various objects
mod sphere;
mod triangle;

// extracting everything we may need
use panel::Panel;
use sphere::Sphere;
use triangle::Triangle;

use super::MaterialBuilder;

/// enum containing all of the object types we can create
pub enum Objects {
    SPHERE(Vec3, f64, Material),
    TRIANGLE(Vec3, Vec3, Vec3, Material),
    PANEL(Vec3, f64, f64, Vec3, Material),
}

impl Objects {
    pub fn create_object(object: Objects) -> Box<dyn Object> {
        match object {
            Objects::SPHERE(position, radius, material) => {
                return Box::new(Sphere::new(position, radius, material));
            }
            Objects::TRIANGLE(vertice_1, vertice_2, vertice_3, material) => {
                return Box::new(Triangle::new(vertice_1, vertice_2, vertice_3, material));
            }
            Objects::PANEL(panel_origin, panel_width, panel_height, mut panel_normal, material) => {
                return Box::new(Panel::new(
                    panel_origin,
                    panel_width,
                    panel_height,
                    panel_normal,
                    material,
                ));
            }
        };
    }
}
