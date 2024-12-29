use crate::collections::{Point, Vector};
use crate::material::Material;
use crate::transform::{Transform, TransformKind, Transformable};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Transform,
    pub material: Material,
}

impl Sphere {
    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = world_point.transform(&self.transform.invert());
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = object_normal.transform(&self.transform.invert().transpose());
        world_normal.normalise()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Transform::new(TransformKind::Identity),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::Angle;
    use crate::transform::Axis;

    #[test]
    fn create_default_sphere() {
        let sphere = Sphere::default();
        let resulting_sphere = Sphere {
            transform: Transform::new(TransformKind::Identity),
            material: Material::default(),
        };
        assert_eq!(sphere, resulting_sphere);
    }

    #[test]
    fn normal_on_unit_sphere() {
        let sphere = Sphere::default();
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
    //     let transform1 = Transform::new(TransformKind::Translate(0.0, 1.0, 0.0));
    //     let transform2 = Transform::from(vec![
    //         TransformKind::Rotate(Axis::Z, Angle::from_radians(std::f64::consts::PI / 5.0)),
    //         TransformKind::Scale(1.0, 0.5, 1.0),
    //     ]);
    //     let sphere1 = Sphere {
    //         transform: transform1,
    //         ..Sphere::default()
    //     };
    //     let sphere2 = Sphere {
    //         transform: transform2,
    //         ..Sphere::default()
    //     };
    //     let point1 = Point::new(0.0, 1.0 + 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //     let point2 = Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //     let normal1 = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //     let normal2 = Vector::new(0.0, 0.97014, -0.24254);
    //     assert_eq!(sphere1.normal_at(point1), normal1);
    //     assert_eq!(sphere2.normal_at(point2), normal2);
    // }
}
