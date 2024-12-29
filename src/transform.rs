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
}

impl Transform {
    pub fn new(transform_kind: TransformKind) -> Transform {
        match transform_kind {
            TransformKind::Identity => Transform::identity(),
        }
    }
}

impl Transform {
    fn identity() -> Transform {
        let vec2d: Vec<Vec<f64>> = IDENTITY.iter().map(|row| row.to_vec()).collect();
        Transform(Matrix::from(&vec2d))
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
}
