use std::ops::{Add, Sub};

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
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
    fn sub_two_vectors() {
        let vector1 = Vector::new(9.0, 12.0, 13.0);
        let vector2 = Vector::new(1.0, 2.0, 3.0);
        let resulting_vector = Vector::new(8.0, 10.0, 10.0);
        assert_eq!(vector1 - vector2, resulting_vector);
    }
}
