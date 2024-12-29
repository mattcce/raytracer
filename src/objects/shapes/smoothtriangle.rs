use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use crate::collections::{Point, Vector};
use crate::objects::{Group, GroupTransformable, Material, Ray, Shape, Transform};
use crate::utils::EPSILON;

#[derive(Debug)]
pub struct SmoothTriangle {
    pub transform: Transform,
    pub material: Material,
    parent: Option<Rc<RefCell<Group>>>,
    pub(crate) vertices: [Point; 3],
    pub(crate) normals: [Vector; 3],
    edges: [Vector; 2],
}

impl SmoothTriangle {
    pub fn new(
        v1: Point,
        v2: Point,
        v3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
    ) -> SmoothTriangle {
        let e1 = v2 - v1;
        let e2 = v3 - v1;
        SmoothTriangle {
            transform: Transform::default(),
            material: Material::default(),
            parent: None,
            vertices: [v1, v2, v3],
            edges: [e1, e2],
            normals: [n1, n2, n3],
        }
    }
}

impl GroupTransformable for SmoothTriangle {
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

impl Shape for SmoothTriangle {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_normal_at(&self, _local_point: Point, uv_coordinates: Option<(f64, f64)>) -> Vector {
        let [n1, n2, n3] = self.normals;
        let (u, v) = uv_coordinates.unwrap();
        n2 * u + n3 * v + n1 * (1.0 - u - v)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<(f64, Option<(f64, f64)>)> {
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
        vec![(t, Some((u, v)))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn intersection_collects_uv_coordinates() {
    //     let smooth_triangle = SmoothTriangle::new(
    //         Point::new(0.0, 1.0, 0.0),
    //         Point::new(-1.0, 0.0, 0.0),
    //         Point::new(1.0, 0.0, 0.0),
    //         Vector::new(0.0, 1.0, 0.0),
    //         Vector::new(-1.0, 0.0, 0.0),
    //         Vector::new(1.0, 0.0, 0.0),
    //     );
    //     let ray = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
    //     let intersections = smooth_triangle.local_intersect(&ray);
    //     let (u, v) = intersections[0].1.unwrap();
    //     assert_eq!(u, 0.45);
    //     assert_eq!(v, 0.25);
    // }

    // #[test]
    // fn smooth_triangle_interpolates_normals() {
    //     let smooth_triangle = SmoothTriangle::new(
    //         Point::new(0.0, 1.0, 0.0),
    //         Point::new(-1.0, 0.0, 0.0),
    //         Point::new(1.0, 0.0, 0.0),
    //         Vector::new(0.0, 1.0, 0.0),
    //         Vector::new(-1.0, 0.0, 0.0),
    //         Vector::new(1.0, 0.0, 0.0),
    //     );
    //     let normal = smooth_triangle.normal_at(Point::new(0.0, 0.0, 0.0), Some((0.45, 0.25)));
    //     let resulting_normal = Vector::new(-0.5547, 0.83205, 0.0);
    //     assert_eq!(normal, resulting_normal);
    // }

    // use crate::scenes::World;

    // #[test]
    // fn intersection_retrieves_interpolated_normal() {
    //     let smooth_triangle = SmoothTriangle::new(
    //         Point::new(0.0, 1.0, 0.0),
    //         Point::new(-1.0, 0.0, 0.0),
    //         Point::new(1.0, 0.0, 0.0),
    //         Vector::new(0.0, 1.0, 0.0),
    //         Vector::new(-1.0, 0.0, 0.0),
    //         Vector::new(1.0, 0.0, 0.0),
    //     );
    //     let ray = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
    //     let normal = World::from(smooth_triangle)
    //         .intersect_ray(&ray)
    //         .hit()
    //         .unwrap()
    //         .precompute()
    //         .normal;
    //     let resulting_normal = Vector::new(-0.5547, 0.83205, 0.0);
    //     assert_eq!(normal, resulting_normal);
    // }
}
