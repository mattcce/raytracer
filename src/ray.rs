use crate::collections::{Point, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
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
}
