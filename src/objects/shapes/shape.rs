use std::fmt::Debug;

use crate::collections::{Point, Vector};
use crate::objects::{Intersectable, Intersections, Material, RawIntersect, Ray, Transform, Transformable};

pub trait Shape: Debug {
    fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = world_point.transform(&self.transformation_matrix().invert());
        let local_normal = self.local_normal_at(local_point);
        let world_normal =
            local_normal.transform(&self.transformation_matrix().invert().transpose());
        world_normal.normalise()
    }

    fn material(&self) -> &Material;
    fn transformation_matrix(&self) -> &Transform;
    fn local_normal_at(&self, local_point: Point) -> Vector;
    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<f64>>;
}

impl<S: Shape + ?Sized> Intersectable<S> for S {
    fn intersect<'a>(&'a self, world_ray: &'a Ray) -> Intersections<'a, S> {
        let local_ray = world_ray.transform(&self.transformation_matrix().invert());
        match self.local_intersect(&local_ray) {
            None => Intersections::default(),
            Some(intersects) => intersects
                .into_iter()
                .map(|t| RawIntersect::new(t, self, world_ray))
                .collect::<Vec<RawIntersect<S>>>()
                .into(),
        }
    }
}