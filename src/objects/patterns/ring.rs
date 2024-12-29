use crate::collections::{Colour, Point};
use crate::objects::{Pattern, Transform};

#[derive(Clone, Debug, PartialEq)]
pub struct Ring {
    pub colour1: Colour,
    pub colour2: Colour,
    pub transform: Transform,
}

impl Ring {
    pub fn new(colour1: Colour, colour2: Colour, transform: Transform) -> Ring {
        Ring {
            colour1,
            colour2,
            transform,
        }
    }
}

impl Pattern for Ring {
    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn local_colour_at(&self, pattern_point: Point) -> Colour {
        let squared_magnitude = pattern_point.x.powi(2) + pattern_point.z.powi(2);
        match (squared_magnitude.sqrt().floor() as i32).rem_euclid(2) {
            x if x == 0 => self.colour1,
            x if x == 1 => self.colour2,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ring_pattern() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let stripe_pattern = Ring::new(colour1, colour2, Transform::default());
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(1.0, 0.0, 0.0)), colour2);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 1.0)), colour2);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.708, 0.0, 0.708)), colour2);
    }
}