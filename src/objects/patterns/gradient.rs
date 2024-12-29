use crate::collections::{Colour, Point};
use crate::objects::{Pattern, Transform};

#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    pub colour1: Colour,
    pub colour2: Colour,
    pub transform: Transform,
}

impl Gradient {
    pub fn new(colour1: Colour, colour2: Colour, transform: Transform) -> Gradient {
        Gradient {
            colour1,
            colour2,
            transform,
        }
    }
}

impl Pattern for Gradient {
    fn frame_transformation(&self) -> &Transform {
        &self.transform
    }

    fn local_colour_at(&self, pattern_point: Point) -> Colour {
        let colour_x = self.colour1;
        colour_x + (self.colour2 - self.colour1) * (pattern_point.x - pattern_point.x.floor())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_gradient_pattern() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let gradient_pattern = Gradient::new(colour1, colour2, Transform::default());
        let resulting_gradient_pattern = Gradient {
            colour1,
            colour2,
            transform: Transform::default(),
        };
        assert_eq!(gradient_pattern, resulting_gradient_pattern);
    }

    #[test]
    fn gradient_pattern_colours() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let gradient_pattern = Gradient::new(colour1, colour2, Transform::default());
        assert_eq!(
            gradient_pattern.colour_at(Point::new(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            gradient_pattern.colour_at(Point::new(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            gradient_pattern.colour_at(Point::new(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }
}
