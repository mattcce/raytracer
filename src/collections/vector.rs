use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{Matrix, Point, Tuple4};

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

    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Self::Output {
        other * self
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Self::Output {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vector {
    pub fn magnitude(self) -> f64 {
        (self.x.powf(2.0_f64) + self.y.powf(2.0_f64) + self.z.powf(2.0_f64)).sqrt()
    }

    pub fn normalise(self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot(self, other: Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn reflect(self, normal: Vector) -> Vector {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl Tuple4 for Vector {
    fn to_tuple4(self) -> [f64; 4] {
        [self.x, self.y, self.z, 0.0]
    }
}

impl From<Matrix> for Vector {
    fn from(matrix: Matrix) -> Self {
        assert_eq!(matrix.rows(), 4);
        assert_eq!(matrix.cols(), 1);

        Vector::new(matrix[[0, 0]], matrix[[1, 0]], matrix[[2, 0]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::floats::approx_eq;

    #[test]
    fn create_zero_vector() {
        let vector = Vector::zero();
        let resulting_vector = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(vector, resulting_vector);
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

    #[test]
    fn mul_scalar_by_vector() {
        let scalar = 2.0_f64;
        let vector = Vector::new(5.0, 5.0, 7.0);
        let resulting_vector = Vector::new(10.0, 10.0, 14.0);
        assert_eq!(scalar * vector, resulting_vector);
    }

    #[test]
    fn mul_vector_by_scalar() {
        let vector = Vector::new(3.0, 1.0, 7.0);
        let scalar = 3.0_f64;
        let resulting_vector = Vector::new(9.0, 3.0, 21.0);
        assert_eq!(vector * scalar, resulting_vector);
    }

    #[test]
    fn div_vector_by_scalar() {
        let vector = Vector::new(3.0, 6.0, 9.0);
        let scalar = 3.0_f64;
        let resulting_vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector / scalar, resulting_vector);
    }

    #[test]
    fn magnitude_of_vector() {
        let vector = Vector::new(1.0, 2.0, 2.0);
        let resulting_magnitude = 3.0_f64;
        assert_eq!(vector.magnitude(), resulting_magnitude);
    }

    #[test]
    fn normalise_vector() {
        let vector = Vector::new(2.0, 3.0, 6.0);
        let resulting_vector = Vector::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0);
        assert_eq!(vector.normalise(), resulting_vector);
    }

    #[test]
    fn dot_product_two_vectors() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        let resulting_value = 20.0_f64;
        assert_eq!(vector1.dot(vector2), resulting_value);
    }

    #[test]
    fn cross_product_two_vectors() {
        let vector1 = Vector::new(2.0, 3.0, 4.0);
        let vector2 = Vector::new(1.0, 2.0, 3.0);
        let resulting_vector = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(vector1.cross(vector2), resulting_vector);
    }

    #[test]
    fn vector_to_tuple4() {
        let vector = Vector::new(7.0, 5.0, 3.0);
        let resulting_tuple4 = [7.0, 5.0, 3.0, 0.0];
        assert_eq!(vector.to_tuple4(), resulting_tuple4);
    }

    #[test]
    fn matrix_to_vector() {
        let vector = Vector::new(2.0, 6.0, 3.0);
        let matrix = Matrix::from(vector);
        assert_eq!(Vector::from(matrix), vector);
    }

    #[test]
    fn reflect_vector() {
        let vector1 = Vector::new(1.0, -1.0, 0.0);
        let vector2 = Vector::new(0.0, -1.0, 0.0);
        let normal1 = Vector::new(0.0, 1.0, 0.0);
        let normal2 = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let resulting_vector1 = Vector::new(1.0, 1.0, 0.0);
        let resulting_vector2 = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(vector1.reflect(normal1), resulting_vector1);

        let vector2_reflected = vector2.reflect(normal2);
        approx_eq!(vector2_reflected.x, resulting_vector2.x);
        approx_eq!(vector2_reflected.y, resulting_vector2.y);
        approx_eq!(vector2_reflected.z, resulting_vector2.z);
    }
}
