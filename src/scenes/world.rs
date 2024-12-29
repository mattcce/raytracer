use crate::collections::{Colour, Point};
use crate::objects::*;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere {
            transform: Transform::new(TransformKind::Identity),
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
        };
        let s2 = Sphere {
            transform: Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            material: Material::default(),
        };
        let light = PointLight::new(Point::new(-10.0, -10.0, -10.0), Colour::new(1.0, 1.0, 1.0));

        World {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world() {
        let world = World::new();
        let resulting_world = World {
            objects: vec![],
            lights: vec![],
        };
        assert_eq!(world, resulting_world);
    }

    #[test]
    fn create_default_world() {
        let default_world = World::default();
        let s1 = Sphere {
            transform: Transform::new(TransformKind::Identity),
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
        };
        let s2 = Sphere {
            transform: Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            material: Material::default(),
        };
        let light = PointLight::new(Point::new(-10.0, -10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        assert_eq!(default_world, resulting_world);
    }
}
