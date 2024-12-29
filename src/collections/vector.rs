use super::Point;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        other + self
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
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
    fn add_two_vectors() {
        let vector1 = Vector::new(4.5, 3.5, 6.0);
        let vector2 = Vector::new(1.5, 2.5, 6.5);
        let resulting_vector = Vector::new(6.0, 6.0, 12.5);
        assert_eq!(vector1 + vector2, resulting_vector);
    }

    #[test]
    fn add_vector_and_point() {
        let vector = Vector::new(1.0, 5.0, -3.0);
        let point = Point::new(1.0, 5.0, -3.0);
        let resulting_point = Point::new(2.0, 10.0, -6.0);
        assert_eq!(vector + point, resulting_point);
    }

    #[test]
    fn sub_two_vectors() {
        let vector1 = Vector::new(9.0, 12.0, 13.0);
        let vector2 = Vector::new(1.0, 2.0, 3.0);
        let resulting_vector = Vector::new(8.0, 10.0, 10.0);
        assert_eq!(vector1 - vector2, resulting_vector);
    }

    #[test]
    fn negate_vector() {
        let vector = Vector::new(7.0, 4.0, 7.0);
        let resulting_vector = Vector::new(-7.0, -4.0, -7.0);
        assert_eq!(-vector, resulting_vector);
    }
}
