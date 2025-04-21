use crate::canvas::RGB;
use crate::world::Ray;
use crate::world::Vec3;

/// module implementing a texture
mod texture;

use texture::Texture;

/// an object abstracting the materials of the world's objects
///
/// An instance of a _Material_ can be used to implement the material properties of an object in
/// the world
///
/// # Examples
///
/// Create a new Material with a black color and some reflectiveness
/// and print all of it's properties
///
/// ```no_run
/// use ray_tracing::Material;
///
/// fn main() {
///     let material: Material = Material::new(
///         RGB::new(0, 0, 0),
///         None,
///         Some(0.8),
///         None,
///         None,
///         None,
///     );
///
///     println!("Color: {:?}", material.get_color());
///     println!("Texture: {:?}", material.get_texture());
///     println!("Reflectiveness: {:?}", material.get_reflectiveness());
///     println!("Specularity: {:?}", material.get_specularity());
///     println!("Refraction: {:?}", material.get_refraction());
///     println!("Transparency: {:?}", material.get_transparency());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Material {
    color: RGB,
    texture: Option<Texture>,
    reflectiveness: Option<f64>,
    specularity: Option<f64>,
    refraction: Option<f64>,
    transparency: Option<f64>,
}

impl Material {
    /// creates a new Material
    pub fn new(
        color: RGB,
        texture_path: Option<&str>,
        reflectiveness: Option<f64>,
        specularity: Option<f64>,
        refraction: Option<f64>,
        transparency: Option<f64>,
    ) -> Self {
        let reflectiveness = match reflectiveness {
            None => None,
            Some(reflectiveness) => Some(reflectiveness.clamp(0.0, 1.0)),
        };
        let texture = match texture_path {
            Some(texture_path) => Some(Texture::load(texture_path)),
            None => None,
        };

        // if we have refraction then trasparency will be full only when `transparency` is not
        // given
        let transparency = if let Some(transparency) = transparency {
            Some(transparency.clamp(0.0, 1.0))
        } else if let Some(refraction) = refraction {
            Some(1.0)
        } else {
            None
        };

        return Self {
            color,
            texture,
            reflectiveness,
            specularity,
            refraction,
            transparency,
        };
    }

    /// retrieves the material's texture
    pub fn get_texture(&self) -> &Option<Texture> {
        return &self.texture;
    }

    /// retrieves the material's specularity value
    pub fn get_specularity(&self) -> &Option<f64> {
        return &self.specularity;
    }

    /// retrieves the material's reflectiveness
    pub fn get_reflectiveness(&self) -> &Option<f64> {
        return &self.reflectiveness;
    }

    /// retrieves the material's color
    pub fn get_color(&self) -> &RGB {
        return &self.color;
    }

    /// retrieves the material's refraction index
    pub fn get_refraction(&self) -> &Option<f64> {
        return &self.refraction;
    }

    /// retrieve the material's transparency level
    pub fn get_transparency(&self) -> &Option<f64> {
        return &self.transparency;
    }
}
