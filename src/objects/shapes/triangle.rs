use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use crate::collections::{Point, Vector};
use crate::objects::{Group, GroupTransformable, Material, Ray, Shape, Transform};
use crate::utils::EPSILON;

#[derive(Debug)]
pub struct Triangle {
    pub transform: Transform,
    pub material: Material,
    parent: Option<Rc<RefCell<Group>>>,
    pub(crate) vertices: [Point; 3],
    edges: [Vector; 2],
    normal: Vector,
}

impl Triangle {
    pub fn new(v1: Point, v2: Point, v3: Point) -> Triangle {
        let e1 = v2 - v1;
        let e2 = v3 - v1;
        let normal = e2.cross(e1).normalise();
        Triangle {
            transform: Transform::default(),
            material: Material::default(),
            parent: None,
            vertices: [v1, v2, v3],
            edges: [e1, e2],
            normal,
        }
    }
}

impl GroupTransformable for Triangle {
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

impl Shape for Triangle {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_normal_at(&self, _local_point: Point) -> Vector {
        self.normal
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let dir_cross_e2 = local_ray.direction.cross(self.edges[1]);
        let det = self.edges[0].dot(dir_cross_e2);
        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = local_ray.origin - self.vertices[0];
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edges[0]);
        let v = f * local_ray.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * self.edges[1].dot(origin_cross_e1);
        vec![t]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_ray_parallel_to_triangle() {
        let triangle = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let ray = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let triangle = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let ray = Ray::new(Point::new(1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let triangle = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let ray = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let triangle = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let ray = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_intersects_triangle() {
        let triangle = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let ray = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::new(0.0, 0.0, 1.0));
        let t_values = triangle.local_intersect(&ray);
        assert_eq!(t_values.len(), 1);
        assert_eq!(t_values[0], 2.0);
    }
}
