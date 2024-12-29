use super::Vector;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_point_and_vector() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(5.0, 6.0, 7.0);
        let resulting_vector = Point::new(6.0, 8.0, 10.0);
        assert_eq!(point + vector, resulting_vector);
    }

    #[test]
    fn sub_two_points() {
        let point1 = Point::new(1.5, 2.5, 3.5);
        let point2 = Point::new(0.5, 0.5, 0.5);
        let resulting_vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(point1 - point2, resulting_vector);
    }

    #[test]
    fn sub_vector_from_point() {
        let point = Point::new(2.0, 3.0, 4.5);
        let vector = Vector::new(1.0, 2.0, 3.0);
        let resulting_point = Point::new(1.0, 1.0, 1.5);
        assert_eq!(point - vector, resulting_point);
    }

    #[test]
    fn negate_point() {
        let point = Point::new(2.0, 5.0, 9.0);
        let resulting_point = Point::new(-2.0, -5.0, -9.0);
        assert_eq!(-point, resulting_point);
    }
}
