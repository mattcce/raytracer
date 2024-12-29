use std::ops::Add;

use crate::collections::Point;
use crate::objects::{Ray, Transform, Transformable};
use crate::utils::EPSILON;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    x_range: [f64; 2],
    y_range: [f64; 2],
    z_range: [f64; 2],
}

impl BoundingBox {
    const UNBOUNDED: BoundingBox = BoundingBox {
        x_range: [f64::NEG_INFINITY, f64::INFINITY],
        y_range: [f64::NEG_INFINITY, f64::INFINITY],
        z_range: [f64::NEG_INFINITY, f64::INFINITY],
    };

    pub const fn new_unbounded() -> BoundingBox {
        BoundingBox::UNBOUNDED
    }

    pub fn from_anchors(anchors: Vec<Point>) -> BoundingBox {
        if anchors.is_empty() {
            return BoundingBox::new_unbounded();
        }

        let Point { x, y, z } = anchors[0];
        let mut x_range = [x, x];
        let mut y_range = [y, y];
        let mut z_range = [z, z];

        for anchor in anchors.iter().skip(1) {
            x_range[0] = f64::min(x_range[0], anchor.x);
            x_range[1] = f64::max(x_range[1], anchor.x);
            y_range[0] = f64::min(y_range[0], anchor.y);
            y_range[1] = f64::max(y_range[1], anchor.y);
            z_range[0] = f64::min(z_range[0], anchor.z);
            z_range[1] = f64::max(z_range[1], anchor.z);
        }

        BoundingBox::from_axial_bounds(x_range, y_range, z_range)
    }

    pub const fn from_axial_bounds(
        x_range: [f64; 2],
        y_range: [f64; 2],
        z_range: [f64; 2],
    ) -> BoundingBox {
        BoundingBox {
            x_range,
            y_range,
            z_range,
        }
    }

    pub fn bound_in_x_axis(mut self, axial_bounds: [f64; 2]) -> BoundingBox {
        let axial_bounds = if axial_bounds[0] > axial_bounds[1] {
            [axial_bounds[1], axial_bounds[0]]
        } else {
            axial_bounds
        };
        self.x_range = axial_bounds;
        self
    }

    pub fn bound_in_y_axis(mut self, axial_bounds: [f64; 2]) -> BoundingBox {
        let axial_bounds = if axial_bounds[0] > axial_bounds[1] {
            [axial_bounds[1], axial_bounds[0]]
        } else {
            axial_bounds
        };
        self.y_range = axial_bounds;
        self
    }

    pub fn bound_in_z_axis(mut self, axial_bounds: [f64; 2]) -> BoundingBox {
        let axial_bounds = if axial_bounds[0] > axial_bounds[1] {
            [axial_bounds[1], axial_bounds[0]]
        } else {
            axial_bounds
        };
        self.z_range = axial_bounds;
        self
    }

    pub fn anchors(&self) -> Vec<Point> {
        let mut anchors = Vec::with_capacity(8);
        for x in self.x_range {
            for y in self.y_range {
                for z in self.z_range {
                    anchors.push(Point::new(x, y, z));
                }
            }
        }
        anchors
    }

    pub fn axial_bounds(&self) -> ([f64; 2], [f64; 2], [f64; 2]) {
        (self.x_range, self.y_range, self.z_range)
    }

    pub fn is_bounded(&self) -> bool {
        // a bounding box is bounded if it does not include all representable points
        // in other words, at least one of the above f64 values must be non-infinite
        !(self.x_range == [f64::NEG_INFINITY, f64::INFINITY]
            && self.y_range == [f64::NEG_INFINITY, f64::INFINITY]
            && self.z_range == [f64::NEG_INFINITY, f64::INFINITY])
    }

    pub fn intersect_bounds<'world: 'ray, 'ray>(
        &'world self,
        ray: &'ray Ray,
        transform_stack: &Vec<&'ray Transform>,
    ) -> bool {
        fn check_axis(range: [f64; 2], origin: f64, direction: f64) -> (f64, f64) {
            assert!(range[0] <= range[1]);

            let [min, max] = range;
            let tmin_numerator = min - origin;
            let tmax_numerator = max - origin;

            let tmin;
            let tmax;
            if direction.abs() >= EPSILON {
                tmin = tmin_numerator / direction;
                tmax = tmax_numerator / direction;
            } else {
                tmin = tmin_numerator * f64::INFINITY;
                tmax = tmax_numerator * f64::INFINITY;
            }

            if tmin > tmax {
                (tmax, tmin)
            } else {
                (tmin, tmax)
            }
        }

        let ray = super::shape::transform_through_stack_forwards(*ray, transform_stack);

        let (xtmin, xtmax) = check_axis(self.x_range, ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(self.y_range, ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(self.z_range, ray.origin.z, ray.direction.z);

        let tmin = [xtmin, ytmin, ztmin].into_iter().reduce(f64::max).unwrap();
        let tmax = [xtmax, ytmax, ztmax].into_iter().reduce(f64::min).unwrap();

        tmax >= tmin
    }
}

impl Add for BoundingBox {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x_range = [
            f64::min(self.x_range[0], rhs.x_range[0]),
            f64::max(self.x_range[1], rhs.x_range[1]),
        ];
        let y_range = [
            f64::min(self.y_range[0], rhs.y_range[0]),
            f64::max(self.y_range[1], rhs.y_range[1]),
        ];
        let z_range = [
            f64::min(self.z_range[0], rhs.z_range[0]),
            f64::max(self.z_range[1], rhs.z_range[1]),
        ];

        BoundingBox::from_axial_bounds(x_range, y_range, z_range)
    }
}

impl Transformable for BoundingBox {
    fn transform(self, transform: &Transform) -> BoundingBox {
        let old_anchors = self.anchors();
        let new_anchors = old_anchors
            .iter()
            .filter(|point| !point.at_infinity())
            .map(|point| point.transform(transform))
            .collect();
        BoundingBox::from_anchors(new_anchors)
    }
}

// Helper enum type for wrapping BoundingBox for ergonomic use. Access to the
// underlying bounding box is still available via a method, but this type is
// generally immutable once constructed. It delegates functions for ray-bbox
// intersections to the internal bbox.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bounds {
    Checked(BoundingBox),
    Unchecked(BoundingBox),
}

impl Bounds {
    pub fn new(bounding_box: BoundingBox) -> Bounds {
        Bounds::Unchecked(bounding_box)
    }

    pub fn raise(self) -> Bounds {
        if let Bounds::Unchecked(bounding_box) = self {
            Bounds::Checked(bounding_box)
        } else {
            self
        }
    }

    pub fn lower(self) -> Bounds {
        if let Bounds::Checked(bounding_box) = self {
            Bounds::Unchecked(bounding_box)
        } else {
            self
        }
    }

    // generally shouldn't be used unless one wants to construct a new bbox and/
    // or inspect the internal bbox for whatever reason; using this to feed rays
    // to the bbox directly instead of going through Bounds is not idiomatic
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Bounds::Checked(bbox) => bbox,
            Bounds::Unchecked(bbox) => bbox,
        }
        .to_owned()
    }

    pub fn intersect_bounds<'world: 'ray, 'ray>(
        &'world self,
        ray: &'ray Ray,
        transform_stack: &Vec<&'ray Transform>,
    ) -> bool {
        match self {
            Bounds::Checked(bbox) => bbox.intersect_bounds(ray, transform_stack),
            Bounds::Unchecked(_) => true,
        }
    }
}

pub trait Bounded {
    fn bounds(&self) -> &Bounds;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_bounded_bounding_box() {
        let bounding_box = BoundingBox::from_anchors(vec![
            Point::new(1.0, 1.0, 1.0),
            Point::new(-1.0, -1.0, -1.0),
        ]);

        assert!(bounding_box.is_bounded());
    }

    #[test]
    fn make_unbounded_bounding_box() {
        let bounding_box = BoundingBox::new_unbounded();

        assert!(!bounding_box.is_bounded());
    }

    #[test]
    fn bound_unbounded_bounding_box() {
        let bounding_box = BoundingBox::new_unbounded()
            .bound_in_x_axis([10.0, 10.0])
            .bound_in_y_axis([0.0, 10.0])
            .bound_in_z_axis([10.0, 0.0]);

        assert!(bounding_box.is_bounded());
    }

    use crate::collections::Angle;
    use crate::objects::{Axis, Transform, TransformKind};
    use crate::utils::floats::approx_eq;
    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn transform_bounding_box() {
        let transform = Transform::from(vec![
            TransformKind::Rotate(Axis::Y, Angle::from_radians(FRAC_PI_4)),
            TransformKind::Rotate(Axis::X, Angle::from_radians(FRAC_PI_4)),
        ]);
        let bounding_box = BoundingBox::from_anchors(vec![
            Point::new(1.0, 1.0, 1.0),
            Point::new(-1.0, -1.0, -1.0),
        ])
        .transform(&transform);

        let BoundingBox {
            x_range,
            y_range,
            z_range,
            ..
        } = bounding_box;
        approx_eq!(x_range[0], -1.414214);
        approx_eq!(x_range[1], 1.414214);
        approx_eq!(y_range[0], -1.707107);
        approx_eq!(y_range[1], 1.707107);
        approx_eq!(z_range[0], -1.707107);
        approx_eq!(z_range[1], 1.707107);
    }

    use crate::collections::Vector;

    #[test]
    fn check_ray_with_bounding_box() {
        let origins = vec![
            Point::new(5.0, 0.5, 0.0),
            Point::new(-5.0, 0.5, 0.0),
            Point::new(0.5, 5.0, 0.0),
            Point::new(0.5, -5.0, 0.0),
            Point::new(0.5, 0.0, 5.0),
            Point::new(0.5, 0.0, -5.0),
            Point::new(0.0, 0.5, 0.0),
            Point::new(-2.0, 0.0, 0.0),
            Point::new(0.0, -2.0, 0.0),
            Point::new(0.0, 0.0, -2.0),
            Point::new(2.0, 0.0, 2.0),
            Point::new(0.0, 2.0, 2.0),
            Point::new(2.0, 2.0, 0.0),
        ];
        let directions = vec![
            Vector::new(-1.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, -1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(2.0, 4.0, 6.0),
            Vector::new(6.0, 2.0, 4.0),
            Vector::new(4.0, 6.0, 2.0),
            Vector::new(0.0, 0.0, -1.0),
            Vector::new(0.0, -1.0, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
        ];
        let rays: Vec<Ray> = origins
            .into_iter()
            .zip(directions.into_iter())
            .map(|(origin, direction)| Ray::new(origin, direction))
            .collect();
        let results = vec![
            true, true, true, true, true, true, true, false, false, false, false, false, false,
        ];
        let bounding_box = BoundingBox::from_anchors(vec![
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        ]);

        for (ray, result) in rays.into_iter().zip(results.into_iter()) {
            println!("{:?}, {:?}", ray, result);
            assert_eq!(bounding_box.intersect_bounds(&ray, &vec![]), result);
        }
    }
}
