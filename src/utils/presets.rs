use crate::collections::{Colour, Point};
use crate::objects::{Material, Pattern, Sphere, Transform};

pub trait Preset: Default {
    fn preset() -> Self {
        Self::default()
    }
}

impl Sphere {
    pub fn glass_sphere() -> Sphere {
        Sphere {
            material: Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Material::preset()
            },
            ..Sphere::preset()
        }
    }
}

#[derive(Debug)]
pub struct TestPattern {
    pub transform: Transform,
}

impl Pattern for TestPattern {
    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn local_colour_at(&self, Point { x, y, z }: Point) -> Colour {
        Colour::new(x, y, z)
    }
}
