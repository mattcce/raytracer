use crate::collections::Colour;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    colour: Colour,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            colour: Colour::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
