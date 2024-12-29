use crate::collections::matrix::Matrix;

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
    Translation(f64, f64, f64),
}

impl Transform {
    pub fn new(transform_kind: TransformKind) -> Transform {
        match transform_kind {
            TransformKind::Identity => Transform::identity(),
            TransformKind::Translation(x, y, z) => Transform::translate(x, y, z),
        }
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
    fn create_translation_transform() {
        let transform = Transform::new(TransformKind::Translation(5.0, -3.0, 2.0));
        let resulting_transform = Transform(Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 5.0],
            vec![0.0, 1.0, 0.0, -3.0],
            vec![0.0, 0.0, 1.0, 2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        assert_eq!(transform, resulting_transform);
    }
}
