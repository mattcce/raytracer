use crate::collections::Point;
use crate::objects::{Pattern, Transform};
use crate::prelude::Colour;

#[derive(Clone, Debug, PartialEq)]
pub struct Stripe {
    pub colour1: Colour,
    pub colour2: Colour,
    pub transform: Transform,
}

impl Stripe {
    pub fn new(colour1: Colour, colour2: Colour, transform: Transform) -> Stripe {
        Stripe {
            colour1,
            colour2,
            transform,
        }
    }
}

impl Pattern for Stripe {
    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn local_colour_at(&self, pattern_point: Point) -> Colour {
        match (pattern_point.x.floor() as i32).rem_euclid(2) {
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
    fn create_stripe_pattern() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let stripe_pattern = Stripe::new(colour1, colour2, Transform::default());
        let resulting_stripe_pattern = Stripe {
            colour1,
            colour2,
            transform: Transform::default(),
        };
        assert_eq!(stripe_pattern, resulting_stripe_pattern);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let stripe_pattern = Stripe::new(colour1, colour2, Transform::default());
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 1.0, 0.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 2.0, 0.0)), colour1);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let stripe_pattern = Stripe::new(colour1, colour2, Transform::default());
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 1.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 2.0)), colour1);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let stripe_pattern = Stripe::new(colour1, colour2, Transform::default());
        assert_eq!(stripe_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(0.9, 0.0, 0.0)), colour1);
        assert_eq!(stripe_pattern.colour_at(Point::new(1.0, 0.0, 0.0)), colour2);
        assert_eq!(
            stripe_pattern.colour_at(Point::new(-0.1, 0.0, 0.0)),
            colour2
        );
        assert_eq!(
            stripe_pattern.colour_at(Point::new(-1.0, 0.0, 0.0)),
            colour2
        );
        assert_eq!(
            stripe_pattern.colour_at(Point::new(-1.1, 0.0, 0.0)),
            colour1
        );
    }
}
