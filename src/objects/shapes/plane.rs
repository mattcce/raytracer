use std::default::Default;

use crate::collections::{Point, Vector};
use crate::objects::{Material, Ray, Transform, Shape, LocallyIntersectable};
use crate::utils::{EPSILON, Preset};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Plane {
    pub transform: Transform,
    pub material: Material,
}

impl Plane {
    pub fn new(transform: Transform, material: Material) -> Plane {
        Plane {
            transform,
            material,
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }
}

impl Preset for Plane {
    fn preset() -> Plane {
        Plane {
            transform: Transform::preset(),
            material: Material::preset(),
        }
    }
}

impl LocallyIntersectable for Plane {
    fn local_normal_at(&self, _local_point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<f64>> {
        if local_ray.direction.y.abs() < EPSILON {
            return None;
        }

        let t = -local_ray.origin.y / local_ray.direction.y;
        Some(vec![t])
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::{Point, Vector};
    use crate::objects::Intersectable;

    use super::*;

    #[test]
    fn normal_of_plane() {
        let default_plane = Plane::default();
        let normal1 = default_plane.normal_at(Point::new(0.0, 0.0, 0.0));
        let normal2 = default_plane.normal_at(Point::new(10.0, 0.0, -10.0));
        let normal3 = default_plane.normal_at(Point::new(-5.0, 0.0, 150.0));
        let resulting_vector = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(normal1, resulting_vector);
        assert_eq!(normal2, resulting_vector);
        assert_eq!(normal3, resulting_vector);
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let default_plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = default_plane.intersect(&ray);
        assert_eq!(intersections.0.len(), 0);
    }

    #[test]
    fn intersect_ray_coplanar_to_plane() {
        let default_plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = default_plane.intersect(&ray);
        assert_eq!(intersections.0.len(), 0);
    }

    #[test]
    fn intersect_plane_from_above() {
        let default_plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let intersections = default_plane.intersect(&ray);
        assert_eq!(intersections.0.len(), 1);
        assert_eq!(intersections.0[0].t, 1.0);
        assert_eq!(intersections.0[0].object, &default_plane);
    }

    #[test]
    fn intersect_plane_from_below() {
        let default_plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let intersections = default_plane.intersect(&ray);
        assert_eq!(intersections.0.len(), 1);
        assert_eq!(intersections.0[0].t, 1.0);
        assert_eq!(intersections.0[0].object, &default_plane);
    }
}
