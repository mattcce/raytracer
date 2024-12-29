use crate::collections::{Point, Vector};
use crate::utils::Preset;
use crate::objects::{Shape};

use crate::objects::*;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Transform,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: Transform, material: Material) -> Sphere {
        Sphere {
            transform,
            material,
        }
    }
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        local_point - Point::new(0.0, 0.0, 0.0)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<f64>> {
        let sphere_to_ray = local_ray.origin - Point::zero();
        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        let sqrt_discriminant = discriminant.sqrt();

        if sqrt_discriminant.is_nan() {
            None
        } else {
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            Some(vec![t1, t2])
        }
    }
}

impl Preset for Sphere {
    fn preset() -> Sphere {
        Sphere {
            transform: Transform::preset(),
            material: Material::preset(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // use crate::collections::Angle;
    // use crate::objects::Axis;
    //
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

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
    }

    #[test]
    fn ray_does_not_intersect_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.0.len(), 0);
    }

    #[test]
    fn ray_originates_within_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);
    }

    #[test]
    fn ray_originates_after_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);
    }

    #[test]
    fn ray_intersects_transformed_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            transform: Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)),
            ..Sphere::default()
        };
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn ray_does_not_intersect_transformed_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            transform: Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)),
            ..Sphere::default()
        };
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.0.len(), 0);
    }
}
