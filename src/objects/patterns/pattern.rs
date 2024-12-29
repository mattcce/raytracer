use std::fmt::Debug;

use crate::collections::{Colour, Point};
use crate::objects::{Transform, Transformable};

pub trait Pattern: Debug {
    fn colour_at(&self, shape_point: Point) -> Colour {
        let pattern_point = shape_point.transform(&self.transformation_matrix().invert());
        self.local_colour_at(pattern_point)
    }

    fn transformation_matrix(&self) -> &Transform;
    fn local_colour_at(&self, pattern_point: Point) -> Colour;
}
