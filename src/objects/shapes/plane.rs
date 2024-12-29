use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use crate::collections::{Point, Vector};
use crate::objects::{Group, GroupTransformable, Material, Ray, Shape, Transform};
use crate::utils::{Preset, EPSILON};

#[derive(Default, Debug)]
pub struct Plane {
    pub transform: Transform,
    pub material: Material,
    parent: Option<Rc<RefCell<Group>>>,
}

impl Plane {
    pub fn new(transform: Transform, material: Material) -> Plane {
        Plane {
            transform,
            material,
            parent: None,
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_normal_at(&self, _local_point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let t = -local_ray.origin.y / local_ray.direction.y;
        vec![t]
    }
}

impl GroupTransformable for Plane {
    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn transformation_matrix_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn parent(&self) -> Option<Rc<RefCell<Group>>> {
        Option::clone(&self.parent)
    }

    fn set_parent(&mut self, group: Rc<RefCell<Group>>) {
        self.parent = Some(group);
    }
}

impl Preset for Plane {
    fn preset() -> Plane {
        Plane::new(Transform::preset(), Material::preset())
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
    }

    #[test]
    fn intersect_plane_from_below() {
        let default_plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let intersections = default_plane.intersect(&ray);
        assert_eq!(intersections.0.len(), 1);
        assert_eq!(intersections.0[0].t, 1.0);
    }
}
