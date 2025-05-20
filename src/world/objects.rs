use crate::canvas::RGB;
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
}

impl<'a> ObjectRayIntersection<'a> {
    /// creates a new ObjectRayIntersection object, it's a private function since only
    /// `check_intersection` should be really used
    fn new(ray: Ray, t: f64, object: &'a Box<dyn Object>) -> Self {
        let object_point = ray.calculate_ray_position(t);
        let viewing_vector = ray.get_direction().get_inverse();

        return Self {
            ray,
            t,
            viewing_vector,
            object,
            object_point,
        };
    }

    /// this function checks for an interaction between the given ray and the list of world objects
    pub fn check_intersection(
        ray: Ray,
        objects: &'a Vec<Box<dyn Object>>,
        min_t: f64,
        max_t: f64,
    ) -> Option<Self> {
        let mut smallest_t = f64::MAX;
        let mut hit_object: Option<&Box<dyn Object>> = None;

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

        if let Some(hit_object) = hit_object {
            return Some(Self::new(ray, smallest_t, hit_object));
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

/// modules implementing various objects
mod sphere;
mod triangle;
mod panel;

// extracting everything we may need
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use panel::Panel;
