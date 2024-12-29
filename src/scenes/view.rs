use crate::collections::{Matrix, Point, Vector};
use crate::objects::{Transform, TransformKind};

pub fn view_transform(from: Point, to: Point, up: Vector) -> Transform {
    let forward = (to - from).normalise();
    let upn = up.normalise();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);

    let orientation = Matrix::from(&vec![
        vec![left.x, left.y, left.z, 0.0],
        vec![true_up.x, true_up.y, true_up.z, 0.0],
        vec![-forward.x, -forward.y, -forward.z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);
    Transform::from(orientation).compose(&Transform::new(TransformKind::Translate(
        -from.x, -from.y, -from.z,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_transform_default() {
        let view_transform = view_transform(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::default();
        assert_eq!(view_transform, resulting_transform);
    }

    #[test]
    fn view_transform_pos_z() {
        let view_transform = view_transform(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::new(TransformKind::Scale(-1.0, 1.0, -1.0));
        assert_eq!(view_transform, resulting_transform);
    }

    #[test]
    fn view_transform_translate() {
        let view_transform = view_transform(
            Point::new(0.0, 0.0, 8.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::new(TransformKind::Translate(0.0, 0.0, -8.0));
        assert_eq!(view_transform, resulting_transform);
    }

    // #[test]
    // fn view_transform_arbitrary() {
    //     let view_transform = view_transform(
    //         Point::new(1.0, 3.0, 2.0),
    //         Point::new(4.0, -2.0, 8.0),
    //         Vector::new(1.0, 1.0, 0.0),
    //     );
    //     let resulting_transform = Transform::from(Matrix::from(&vec![
    //         vec![-0.50709, 0.50709, 0.67612, -2.36643],
    //         vec![0.76772, 0.60609, 0.12122, -2.82843],
    //         vec![-0.35857, 0.59761, -0.71714, 0.0],
    //         vec![0.0, 0.0, 0.0, 1.0],
    //     ]));
    //     assert_eq!(view_transform, resulting_transform);
    // }
}
