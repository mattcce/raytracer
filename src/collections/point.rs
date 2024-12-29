use super::{Matrix, Tuple4, Vector};
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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

impl Tuple4 for Point {
    fn to_tuple4(self) -> [f64; 4] {
        [self.x, self.y, self.z, 1.0]
    }
}

impl From<Matrix> for Point {
    fn from(matrix: Matrix) -> Self {
        assert_eq!(matrix.rows(), 4);
        assert_eq!(matrix.cols(), 1);
        // assert_eq!(matrix[[3, 0]], 1.0);

        Point::new(matrix[[0, 0]], matrix[[1, 0]], matrix[[2, 0]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point() {
        let point_new = Point::new(2.0, 3.0, 4.0);
        let point_direct = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(point_new, point_direct);
    }

    #[test]
    fn create_zero_origin() {
        let point = Point::zero();
        let resulting_point = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(point, resulting_point);
    }

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

    #[test]
    fn point_to_tuple4() {
        let point = Point::new(1.0, 3.0, 8.0);
        let resulting_tuple4 = [1.0, 3.0, 8.0, 1.0];
        assert_eq!(point.to_tuple4(), resulting_tuple4);
    }

    #[test]
    fn matrix_to_point() {
        let point = Point::new(1.0, 5.0, 2.0);
        let matrix = Matrix::from(point);
        assert_eq!(Point::from(matrix), point);
    }

    // #[test]
    // #[should_panic]
    // fn non_column_matrix_to_point() {
    //     let point = Point::new(1.0, 5.0, 2.0);
    //     let mut matrix = Matrix::from(point);
    //     matrix[[3, 0]] = 10.0;
    //     assert_eq!(Point::from(matrix), point);
    // }
}
