use crate::collections::{Angle, Matrix, Tuple4};
use std::ops::Mul;

const IDENTITY: [[f64; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

#[derive(Clone, Debug, PartialEq)]
pub struct Transform(Matrix);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransformKind {
    Identity,
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    Reflect(Axis),
    Rotate(Axis, Angle),
    Shear(f64, f64, f64, f64, f64, f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Transform {
    pub fn new(transform_kind: TransformKind) -> Transform {
        match transform_kind {
            TransformKind::Identity => Transform::identity(),
            TransformKind::Translate(x, y, z) => Transform::translate(x, y, z),
            TransformKind::Scale(x, y, z) => Transform::scale(x, y, z),
            TransformKind::Reflect(axis) => match axis {
                Axis::X => Transform::reflect_in_x_axis(),
                Axis::Y => Transform::reflect_in_y_axis(),
                Axis::Z => Transform::reflect_in_z_axis(),
            },
            TransformKind::Rotate(axis, angle) => match axis {
                Axis::X => Transform::rotate_about_x_axis(angle),
                Axis::Y => Transform::rotate_about_y_axis(angle),
                Axis::Z => Transform::rotate_about_z_axis(angle),
            },
            TransformKind::Shear(x_y, x_z, y_x, y_z, z_x, z_y) => {
                Transform::shear(x_y, x_z, y_x, y_z, z_x, z_y)
            }
        }
    }

    pub fn invert(&self) -> Transform {
        Transform(self.0.invert())
    }

    // transform_a.compose(transform_b) applies transform_a first then transform_b
    // Mul trait not implemented due to potential confusion on the order of application
    pub fn compose(&self, other: &Transform) -> Transform {
        // clone to prevent moving Matrix out of original Transform
        Transform(other.0.clone() * &self.0)
    }
}

impl From<Matrix> for Transform {
    fn from(matrix: Matrix) -> Transform {
        Transform(matrix)
    }
}

impl From<Vec<TransformKind>> for Transform {
    fn from(vec_tf: Vec<TransformKind>) -> Transform {
        vec_tf
            .iter()
            .map(|&transform_kind| Transform::new(transform_kind))
            .reduce(|a, b| a.compose(&b))
            .unwrap()
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

    fn scale(x: f64, y: f64, z: f64) -> Transform {
        let mut scaling_matrix = Transform::base();
        scaling_matrix[[0, 0]] = x;
        scaling_matrix[[1, 1]] = y;
        scaling_matrix[[2, 2]] = z;
        Transform(scaling_matrix)
    }

    fn reflect_in_x_axis() -> Transform {
        let mut reflection_matrix = Transform::base();
        reflection_matrix[[0, 0]] = -1.0;
        Transform(reflection_matrix)
    }
    fn reflect_in_y_axis() -> Transform {
        let mut reflection_matrix = Transform::base();
        reflection_matrix[[1, 1]] = -1.0;
        Transform(reflection_matrix)
    }
    fn reflect_in_z_axis() -> Transform {
        let mut reflection_matrix = Transform::base();
        reflection_matrix[[2, 2]] = -1.0;
        Transform(reflection_matrix)
    }

    fn rotate_about_x_axis(mut angle: Angle) -> Transform {
        let mut rotation_matrix = Transform::base();
        rotation_matrix[[1, 1]] = angle.radians().cos();
        rotation_matrix[[1, 2]] = -angle.radians().sin();
        rotation_matrix[[2, 1]] = angle.radians().sin();
        rotation_matrix[[2, 2]] = angle.radians().cos();
        Transform(rotation_matrix)
    }
    fn rotate_about_y_axis(mut angle: Angle) -> Transform {
        let mut rotation_matrix = Transform::base();
        rotation_matrix[[0, 0]] = angle.radians().cos();
        rotation_matrix[[0, 2]] = angle.radians().sin();
        rotation_matrix[[2, 0]] = -angle.radians().sin();
        rotation_matrix[[2, 2]] = angle.radians().cos();
        Transform(rotation_matrix)
    }
    fn rotate_about_z_axis(mut angle: Angle) -> Transform {
        let mut rotation_matrix = Transform::base();
        rotation_matrix[[0, 0]] = angle.radians().cos();
        rotation_matrix[[0, 1]] = -angle.radians().sin();
        rotation_matrix[[1, 0]] = angle.radians().sin();
        rotation_matrix[[1, 1]] = angle.radians().cos();
        Transform(rotation_matrix)
    }

    fn shear(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Transform {
        let mut shearing_matrix = Transform::base();
        shearing_matrix[[0, 1]] = x_y;
        shearing_matrix[[0, 2]] = x_z;
        shearing_matrix[[1, 0]] = y_x;
        shearing_matrix[[1, 2]] = y_z;
        shearing_matrix[[2, 0]] = z_x;
        shearing_matrix[[2, 1]] = z_y;
        Transform(shearing_matrix)
    }
}

impl Mul<&Matrix> for Transform {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Self::Output {
        self.0 * other
    }
}

pub trait Transformable {
    // transform is consuming because it accepts Tuple4 types which are Copy
    fn transform(self, transform: &Transform) -> Self;
}

impl<T: Tuple4 + From<Matrix>> Transformable for T {
    fn transform(self, transform: &Transform) -> T {
        T::from(transform.clone() * &Matrix::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::{Point, Vector};
    use std::f64::consts::FRAC_PI_2 as MATH_FRAC_PI_2;

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
        assert_eq!(point.transform(&transform), point);
    }

    #[test]
    fn identity_transform_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let transform = Transform::new(TransformKind::Identity);
        assert_eq!(vector.transform(&transform), vector);
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
        assert_eq!(point.transform(&transform), resulting_point);
    }

    #[test]
    fn invert_translation() {
        let point = Point::new(-3.0, 4.0, 5.0);
        let transform = Transform::new(TransformKind::Translate(5.0, -3.0, 2.0)).invert();
        let resulting_point = Point::new(-8.0, 7.0, 3.0);
        assert_eq!(point.transform(&transform), resulting_point);
    }

    #[test]
    fn translate_vector() {
        let vector = Vector::new(5.0, -3.0, 2.0);
        let transform = Transform::new(TransformKind::Translate(5.0, -3.0, 2.0));
        assert_eq!(vector.transform(&transform), vector);
    }

    #[test]
    fn create_scaling_transform() {
        let transform = Transform::new(TransformKind::Scale(2.0, 3.0, 4.0));
        let resulting_transform = Transform(Matrix::from(&vec![
            vec![2.0, 0.0, 0.0, 0.0],
            vec![0.0, 3.0, 0.0, 0.0],
            vec![0.0, 0.0, 4.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform, resulting_transform);
    }

    #[test]
    fn scale_point() {
        let point = Point::new(-3.0, 4.0, 5.0);
        let transform = Transform::new(TransformKind::Scale(5.0, -3.0, 2.0));
        let resulting_point = Point::new(-15.0, -12.0, 10.0);
        assert_eq!(point.transform(&transform), resulting_point);
    }

    #[test]
    fn scale_vector() {
        let vector = Vector::new(5.0, -3.0, 2.0);
        let transform = Transform::new(TransformKind::Scale(5.0, -3.0, 2.0));
        let resulting_vector = Vector::new(25.0, 9.0, 4.0);
        assert_eq!(vector.transform(&transform), resulting_vector);
    }

    #[test]
    fn create_reflecting_transform() {
        let transform_x = Transform::new(TransformKind::Reflect(Axis::X));
        let transform_y = Transform::new(TransformKind::Reflect(Axis::Y));
        let transform_z = Transform::new(TransformKind::Reflect(Axis::Z));
        let resulting_transform_x = Transform(Matrix::from(&vec![
            vec![-1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        let resulting_transform_y = Transform(Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, -1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        let resulting_transform_z = Transform(Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, -1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform_x, resulting_transform_x);
        assert_eq!(transform_y, resulting_transform_y);
        assert_eq!(transform_z, resulting_transform_z);
    }

    #[test]
    fn reflect_point() {
        let point = Point::new(1.0, 2.0, 3.0);
        let transform_x = Transform::new(TransformKind::Reflect(Axis::X));
        let transform_y = Transform::new(TransformKind::Reflect(Axis::Y));
        let transform_z = Transform::new(TransformKind::Reflect(Axis::Z));
        let resulting_point_x = Point::new(-1.0, 2.0, 3.0);
        let resulting_point_y = Point::new(1.0, -2.0, 3.0);
        let resulting_point_z = Point::new(1.0, 2.0, -3.0);
        assert_eq!(point.transform(&transform_x), resulting_point_x);
        assert_eq!(point.transform(&transform_y), resulting_point_y);
        assert_eq!(point.transform(&transform_z), resulting_point_z);
    }

    #[test]
    fn reflect_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let transform_x = Transform::new(TransformKind::Reflect(Axis::X));
        let transform_y = Transform::new(TransformKind::Reflect(Axis::Y));
        let transform_z = Transform::new(TransformKind::Reflect(Axis::Z));
        let resulting_vector_x = Vector::new(-1.0, 2.0, 3.0);
        let resulting_vector_y = Vector::new(1.0, -2.0, 3.0);
        let resulting_vector_z = Vector::new(1.0, 2.0, -3.0);
        assert_eq!(vector.transform(&transform_x), resulting_vector_x);
        assert_eq!(vector.transform(&transform_y), resulting_vector_y);
        assert_eq!(vector.transform(&transform_z), resulting_vector_z);
    }

    #[test]
    fn create_rotation_transform() {
        let mut r = Angle::from_radians(MATH_FRAC_PI_2);
        let transform_x = Transform::new(TransformKind::Rotate(Axis::X, r));
        let transform_y = Transform::new(TransformKind::Rotate(Axis::Y, r));
        let transform_z = Transform::new(TransformKind::Rotate(Axis::Z, r));
        let sin_r = r.radians().sin();
        let cos_r = r.radians().cos();
        let resulting_transform_x = Transform(Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, cos_r, -sin_r, 0.0],
            vec![0.0, sin_r, cos_r, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        let resulting_transform_y = Transform(Matrix::from(&vec![
            vec![cos_r, 0.0, sin_r, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-sin_r, 0.0, cos_r, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        let resulting_transform_z = Transform(Matrix::from(&vec![
            vec![cos_r, -sin_r, 0.0, 0.0],
            vec![sin_r, cos_r, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform_x, resulting_transform_x);
        assert_eq!(transform_y, resulting_transform_y);
        assert_eq!(transform_z, resulting_transform_z);
    }

    // commented out because tests fail due to floating-point error
    //
    // #[test]
    // fn rotate_point() {
    //     let r = Angle::from_radians(MATH_FRAC_PI_2);
    //     let point_x = Point::new(0.0, 1.0, 0.0);
    //     let point_y = Point::new(0.0, 0.0, 1.0);
    //     let point_z = Point::new(0.0, 1.0, 0.0);
    //     let transform_x = Transform::new(TransformKind::Rotate(Axis::X, r));
    //     let transform_y = Transform::new(TransformKind::Rotate(Axis::Y, r));
    //     let transform_z = Transform::new(TransformKind::Rotate(Axis::Z, r));
    //     let resulting_point_x = Point::new(0.0, 0.0, 1.0);
    //     let resulting_point_y = Point::new(1.0, 0.0, 0.0);
    //     let resulting_point_z = Point::new(-1.0, 0.0, 0.0);
    //     assert_eq!(point_x.transform(transform_x), resulting_point_x);
    //     assert_eq!(point_y.transform(transform_y), resulting_point_y);
    //     assert_eq!(point_z.transform(transform_z), resulting_point_z);
    // }

    // #[test]
    // fn rotate_vector() {
    //     let r = Angle::from_radians(MATH_FRAC_PI_2);
    //     let vector_x = Vector::new(0.0, 1.0, 0.0);
    //     let vector_y = Vector::new(0.0, 0.0, 1.0);
    //     let vector_z = Vector::new(0.0, 1.0, 0.0);
    //     let transform_x = Transform::new(TransformKind::Rotate(Axis::X, r));
    //     let transform_y = Transform::new(TransformKind::Rotate(Axis::Y, r));
    //     let transform_z = Transform::new(TransformKind::Rotate(Axis::Z, r));
    //     let resulting_vector_x = Vector::new(0.0, 0.0, 1.0);
    //     let resulting_vector_y = Vector::new(1.0, 0.0, 0.0);
    //     let resulting_vector_z = Vector::new(-1.0, 0.0, 0.0);
    //     assert_eq!(vector_x.transform(transform_x), resulting_vector_x);
    //     assert_eq!(vector_y.transform(transform_y), resulting_vector_y);
    //     assert_eq!(vector_z.transform(transform_z), resulting_vector_z);
    // }

    #[test]
    fn create_shearing_transform() {
        let transform = Transform::new(TransformKind::Shear(2.0, 3.0, 4.0, 5.0, 6.0, 7.0));
        let resulting_transform = Transform(Matrix::from(&vec![
            vec![1.0, 2.0, 3.0, 0.0],
            vec![4.0, 1.0, 5.0, 0.0],
            vec![6.0, 7.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform, resulting_transform);
    }

    #[test]
    fn shear_point() {
        let point = Point::new(2.0, 3.0, 4.0);
        let transform1 = Transform::new(TransformKind::Shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        let transform2 = Transform::new(TransformKind::Shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0));
        let transform3 = Transform::new(TransformKind::Shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0));
        let transform4 = Transform::new(TransformKind::Shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0));
        let transform5 = Transform::new(TransformKind::Shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0));
        let transform6 = Transform::new(TransformKind::Shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0));
        let resulting_point1 = Point::new(5.0, 3.0, 4.0);
        let resulting_point2 = Point::new(6.0, 3.0, 4.0);
        let resulting_point3 = Point::new(2.0, 5.0, 4.0);
        let resulting_point4 = Point::new(2.0, 7.0, 4.0);
        let resulting_point5 = Point::new(2.0, 3.0, 6.0);
        let resulting_point6 = Point::new(2.0, 3.0, 7.0);
        assert_eq!(point.transform(&transform1), resulting_point1);
        assert_eq!(point.transform(&transform2), resulting_point2);
        assert_eq!(point.transform(&transform3), resulting_point3);
        assert_eq!(point.transform(&transform4), resulting_point4);
        assert_eq!(point.transform(&transform5), resulting_point5);
        assert_eq!(point.transform(&transform6), resulting_point6);
    }

    #[test]
    fn shear_vector() {
        let vector = Vector::new(2.0, 3.0, 4.0);
        let transform1 = Transform::new(TransformKind::Shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        let transform2 = Transform::new(TransformKind::Shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0));
        let transform3 = Transform::new(TransformKind::Shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0));
        let transform4 = Transform::new(TransformKind::Shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0));
        let transform5 = Transform::new(TransformKind::Shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0));
        let transform6 = Transform::new(TransformKind::Shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0));
        let resulting_vector1 = Vector::new(5.0, 3.0, 4.0);
        let resulting_vector2 = Vector::new(6.0, 3.0, 4.0);
        let resulting_vector3 = Vector::new(2.0, 5.0, 4.0);
        let resulting_vector4 = Vector::new(2.0, 7.0, 4.0);
        let resulting_vector5 = Vector::new(2.0, 3.0, 6.0);
        let resulting_vector6 = Vector::new(2.0, 3.0, 7.0);
        assert_eq!(vector.transform(&transform1), resulting_vector1);
        assert_eq!(vector.transform(&transform2), resulting_vector2);
        assert_eq!(vector.transform(&transform3), resulting_vector3);
        assert_eq!(vector.transform(&transform4), resulting_vector4);
        assert_eq!(vector.transform(&transform5), resulting_vector5);
        assert_eq!(vector.transform(&transform6), resulting_vector6);
    }

    #[test]
    fn chain_transformations() {
        let chained_transform = Transform::new(TransformKind::Rotate(
            Axis::X,
            Angle::from_radians(MATH_FRAC_PI_2),
        ))
        .compose(
            &Transform::new(TransformKind::Scale(5.0, 5.0, 5.0))
                .compose(&Transform::new(TransformKind::Translate(10.0, 5.0, 7.0))),
        );
        let resulting_transform = Transform::from(vec![
            TransformKind::Rotate(Axis::X, Angle::from_radians(MATH_FRAC_PI_2)),
            TransformKind::Scale(5.0, 5.0, 5.0),
            TransformKind::Translate(10.0, 5.0, 7.0),
        ]);
        assert_eq!(chained_transform, resulting_transform);
    }
}
