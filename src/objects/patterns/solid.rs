use crate::collections::{Colour, Point};
use crate::objects::{Pattern, Transform};

#[derive(Clone, Debug, PartialEq)]
pub struct Solid {
    pub colour: Colour,
    pub transform: Transform,
}

impl Solid {
    pub fn new(colour: Colour) -> Solid {
        Solid {
            colour,
            transform: Transform::default(),
        }
    }

    pub(crate) fn preset() -> Solid {
        Solid::new(Colour::new(1.0, 1.0, 1.0))
    }
}

impl Pattern for Solid {
    fn colour_at(&self, _pattern_point: Point) -> Colour {
        // force instant return since no calculation is actually needed
        self.colour
    }

    fn frame_transformation(&self) -> &Transform {
        &self.transform
    }

    fn local_colour_at(&self, _shape_point: Point) -> Colour {
        self.colour
    }
}

impl Default for Solid {
    fn default() -> Solid {
        Solid::new(Colour::new(0.0, 0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_solid() {
        let colour = Colour::new(1.0, 1.0, 1.0);
        let solid = Solid::new(colour);
        let resulting_solid = Solid {
            colour,
            transform: Transform::default(),
        };
        assert_eq!(solid, resulting_solid);
    }

    #[test]
    fn solid_pattern_constant_everywhere() {
        let colour = Colour::new(1.0, 1.0, 1.0);
        let solid = Solid::new(colour);
        assert_eq!(solid.colour_at(Point::new(0.0, 0.0, 0.0)), colour);
        assert_eq!(solid.colour_at(Point::new(100.0, 100.0, 100.0)), colour);
        assert_eq!(solid.colour_at(Point::new(-100.0, -100.0, -100.0)), colour);
    }
}
