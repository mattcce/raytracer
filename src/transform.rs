use crate::collections::{Matrix, Point, Tuple4, Vector};
use std::ops::Mul;

const IDENTITY: [[f64; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

#[derive(Clone, Debug, PartialEq)]
pub struct Transform(Matrix);

pub enum TransformKind {
    Identity,
    Translate(f64, f64, f64),
}

impl Transform {
    pub fn new(transform_kind: TransformKind) -> Transform {
        match transform_kind {
            TransformKind::Identity => Transform::identity(),
            TransformKind::Translate(x, y, z) => Transform::translate(x, y, z),
        }
    }

    pub fn from(matrix: Matrix) -> Transform {
        Transform(matrix)
    }

    pub fn invert(transform: Transform) -> Transform {
        Transform(transform.0.invert())
    }
}

impl Transform {
    fn base() -> Matrix {
        Matrix::from(&IDENTITY.iter().map(|row| row.to_vec()).collect())
    }

    fn identity() -> Transform {
        let base_matrix = Transform::base();
        Transform(base_matrix)
    }

    fn translate(x: f64, y: f64, z: f64) -> Transform {
        let mut translation_matrix = Transform::base();
        translation_matrix[[0, 3]] = x;
        translation_matrix[[1, 3]] = y;
        translation_matrix[[2, 3]] = z;
        Transform(translation_matrix)
    }
}

impl Mul<&Matrix> for Transform {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Self::Output {
        self.0 * other
    }
}

pub trait Transformable<T> {
    // transform is consuming because it accepts Tuple4 types which are Copy
    fn transform(self, transform: Transform) -> T;
}

impl<T: Tuple4 + From<Matrix>> Transformable<T> for T {
    fn transform(self, transform: Transform) -> T {
        T::from(transform.clone() * &Matrix::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_identity_transform() {
        let transform = Transform::new(TransformKind::Identity);
        let resulting_transform = Transform(Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform, resulting_transform);
    }

    #[test]
    fn identity_transform_point() {
        let point = Point::new(1.0, 2.0, 3.0);
        let transform = Transform::new(TransformKind::Identity);
        assert_eq!(point.transform(transform), point);
    }

    #[test]
    fn identity_transform_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let transform = Transform::new(TransformKind::Identity);
        assert_eq!(vector.transform(transform), vector);
    }

    #[test]
    fn create_translation_transform() {
        let transform = Transform::new(TransformKind::Translate(5.0, -3.0, 2.0));
        let resulting_transform = Transform(Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 5.0],
            vec![0.0, 1.0, 0.0, -3.0],
            vec![0.0, 0.0, 1.0, 2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform, resulting_transform);
    }

    #[test]
    fn translate_point() {
        let point = Point::new(-3.0, 4.0, 5.0);
        let transform = Transform::new(TransformKind::Translate(5.0, -3.0, 2.0));
        let resulting_point = Point::new(2.0, 1.0, 7.0);
        assert_eq!(point.transform(transform), resulting_point);
    }

    #[test]
    fn translate_vector() {
        let vector = Vector::new(5.0, -3.0, 2.0);
        let transform = Transform::new(TransformKind::Translate(5.0, -3.0, 2.0));
        assert_eq!(vector.transform(transform), vector);
    }
}
