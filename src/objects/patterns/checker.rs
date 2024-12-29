use crate::collections::{Colour, Point};
use crate::objects::{Pattern, Transform};

#[derive(Clone, Debug, PartialEq)]
pub struct Checker {
    pub colour1: Colour,
    pub colour2: Colour,
    pub transform: Transform,
}

impl Checker {
    pub fn new(colour1: Colour, colour2: Colour, transform: Transform) -> Checker {
        Checker {
            colour1,
            colour2,
            transform,
        }
    }
}

impl Pattern for Checker {
    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn local_colour_at(&self, pattern_point: Point) -> Colour {
        let floored_sum_of_lengths = (pattern_point.x.floor() + pattern_point.y.floor() + pattern_point.z.floor()) as i32;
        match floored_sum_of_lengths.rem_euclid(2) {
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
    fn create_checker_pattern() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let checker_pattern = Checker::new(colour1, colour2, Transform::default());
        let resulting_checker_pattern = Checker {
            colour1,
            colour2,
            transform: Transform::default(),
        };
        assert_eq!(checker_pattern, resulting_checker_pattern);
    }

    #[test]
    fn checker_pattern_repeats_in_x() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let checker_pattern = Checker::new(colour1, colour2, Transform::default());
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(checker_pattern.colour_at(Point::new(0.99, 0.0, 0.0)), colour1);
        assert_eq!(checker_pattern.colour_at(Point::new(1.01, 0.0, 0.0)), colour2);
    }

    #[test]
    fn checker_pattern_repeats_in_y() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let checker_pattern = Checker::new(colour1, colour2, Transform::default());
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 0.99, 0.0)), colour1);
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 1.01, 0.0)), colour2);
    }

    #[test]
    fn checker_pattern_repeats_in_z() {
        let colour1 = Colour::new(1.0, 1.0, 1.0);
        let colour2 = Colour::new(0.0, 0.0, 0.0);
        let checker_pattern = Checker::new(colour1, colour2, Transform::default());
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 0.0, 0.0)), colour1);
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 0.99, 0.0)), colour1);
        assert_eq!(checker_pattern.colour_at(Point::new(0.0, 1.01, 0.0)), colour2);
    }
}