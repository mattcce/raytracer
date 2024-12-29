use crate::collections::{Point, Vector};

use super::{Transform, Transformable};
use super::Intersections;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

pub trait Intersectable {
    fn intersect<'a>(&'a self, ray: &'a Ray) -> Intersections<'a>;
}

impl Transformable for Ray {
    fn transform(self, transform: &Transform) -> Self {
        Ray::new(
            self.origin.transform(transform),
            self.direction.transform(transform),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(6.0, 5.0, 4.0);
        let ray = Ray::new(origin, direction);
        let resulting_ray = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(6.0, 5.0, 4.0),
        };
        assert_eq!(ray, resulting_ray);
    }

    #[test]
    fn ray_position() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }
}
