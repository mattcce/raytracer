use crate::objects::{Pattern, Solid};
use crate::utils::Preset;

#[derive(Debug)]
pub struct Material {
    pub pattern: Box<dyn Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectance: f64,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            pattern: Box::<Solid>::default(),
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflectance: 0.0,
        }
    }
}

impl Preset for Material {
    fn preset() -> Material {
        Material {
            pattern: Box::new(Solid::preset()),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectance: 0.0,
        }
    }
}
