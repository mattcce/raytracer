use crate::collections::{Point, Vector};
use crate::transform::{Transform, TransformKind, Transformable};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: Transform,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Transform::new(TransformKind::Identity),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = world_point.transform(&self.transform.invert());
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = object_normal.transform(&self.transform.invert().transpose());
        world_normal.normalise()
    }
}

impl From<&Transform> for Sphere {
    fn from(transform: &Transform) -> Self {
        Sphere {
            transform: transform.clone(),
        }
    }
}

impl Transformable for Sphere {
    fn transform(self, transform: &Transform) -> Self {
        Sphere {
            transform: self.transform.compose(transform),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::Angle;
    use crate::transform::Axis;

    #[test]
    fn create_sphere() {
        let sphere = Sphere::new();
        let resulting_sphere = Sphere {
            transform: Transform::new(TransformKind::Identity),
        };
        assert_eq!(sphere, resulting_sphere);
    }

    #[test]
    fn create_sphere_from_transform() {
        let transform = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        let sphere = Sphere::from(&transform);
        let resulting_sphere = Sphere {
            transform: Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)),
        };
        assert_eq!(sphere, resulting_sphere);
    }

    #[test]
    fn transform_sphere() {
        let transform = Transform::new(TransformKind::Scale(1.0, 1.0, 5.0));
        let sphere = Sphere::new().transform(&transform);
        let resulting_sphere = Sphere {
            transform: Transform::new(TransformKind::Scale(1.0, 1.0, 5.0)),
        };
        assert_eq!(sphere, resulting_sphere);
    }

    #[test]
    fn normal_on_unit_sphere() {
        let sphere = Sphere::new();
        let point1 = Point::new(1.0, 0.0, 0.0);
        let point2 = Point::new(0.0, 1.0, 0.0);
        let point3 = Point::new(0.0, 0.0, 1.0);
        let point4 = Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        let normal1 = Vector::new(1.0, 0.0, 0.0);
        let normal2 = Vector::new(0.0, 1.0, 0.0);
        let normal3 = Vector::new(0.0, 0.0, 1.0);
        let normal4 = Vector::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(sphere.normal_at(point1), normal1);
        assert_eq!(sphere.normal_at(point2), normal2);
        assert_eq!(sphere.normal_at(point3), normal3);
        assert_eq!(sphere.normal_at(point4), normal4);
    }

    // #[test]
    // fn normal_on_transformed_sphere() {
    //     let sphere1 = Sphere::from(&Transform::new(TransformKind::Translate(0.0, 1.0, 0.0)));
    //     let sphere2 = Sphere::from(&Transform::from(vec![
    //         TransformKind::Rotate(Axis::Z, Angle::from_radians(std::f64::consts::PI / 5.0)),
    //         TransformKind::Scale(1.0, 0.5, 1.0),
    //     ]));
    //     let point1 = Point::new(0.0, 1.0 + 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //     let point2 = Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //     let normal1 = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //     let normal2 = Vector::new(0.0, 0.97014, -0.24254);
    //     assert_eq!(sphere1.normal_at(point1), normal1);
    //     assert_eq!(sphere2.normal_at(point2), normal2);
    // }
}
