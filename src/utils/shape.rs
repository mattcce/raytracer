use crate::collections::{Point, Vector};
use crate::objects::{
    Intersectable, Intersections, Material, RawIntersect, Ray, Transform, Transformable,
};
use std::fmt::Debug;

pub trait Shape: LocallyIntersectable + Debug {
    fn normal_at(&self, world_point: Point) -> Vector;
    fn material(&self) -> &Material;
    fn transformation_matrix(&self) -> &Transform;
}

pub trait LocallyIntersectable {
    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<f64>>;
}

impl<S: Shape> Intersectable for S {
    fn intersect<'a>(&'a self, world_ray: &'a Ray) -> Intersections<'a> {
        let local_ray = world_ray.transform(&self.transformation_matrix().invert());
        match self.local_intersect(&local_ray) {
            None => Intersections::default(),
            Some(intersects) => intersects
                .into_iter()
                .map(|t| RawIntersect::new(t, self, &world_ray))
                .collect::<Vec<RawIntersect>>()
                .into(),
        }
    }
}

impl Intersectable for dyn Shape {
    fn intersect<'a>(&'a self, world_ray: &'a Ray) -> Intersections<'a> {
        let local_ray = world_ray.transform(&self.transformation_matrix().invert());
        match self.local_intersect(&local_ray) {
            None => Intersections::default(),
            Some(intersects) => intersects
                .into_iter()
                .map(|t| RawIntersect::new(t, self, &world_ray))
                .collect::<Vec<RawIntersect>>()
                .into(),
        }
    }
}