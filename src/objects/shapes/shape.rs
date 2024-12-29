use std::fmt::Debug;

use crate::collections::{Point, Vector};
use crate::objects::{
    GroupTransformable, Intersectable, Intersections, Material, RawIntersect, Ray,
};

pub trait Shape: Debug + GroupTransformable {
    fn normal_at(&self, world_point: Point, uv_coordinates: Option<(f64, f64)>) -> Vector {
        let local_point = self.world_to_object_point(world_point);
        let local_normal = self.local_normal_at(local_point, uv_coordinates);
        let world_normal = self.normal_to_world_vector(local_normal);
        world_normal.normalise()
    }

    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn local_normal_at(&self, local_point: Point, uv_coordinates: Option<(f64, f64)>) -> Vector;
    fn local_intersect(&self, local_ray: &Ray) -> Vec<(f64, Option<(f64, f64)>)>;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl<S: Shape + ?Sized> Intersectable for S {
    fn intersect<'a>(&'a self, world_ray: &'a Ray) -> Intersections<'a, Self> {
        let local_ray = self.world_to_object_ray(*world_ray);
        match self.local_intersect(&local_ray) {
            t_values if t_values.is_empty() => Intersections::default(),
            t_values => t_values
                .into_iter()
                .map(|(t, uv_coordinates)| RawIntersect::new(t, self, world_ray, uv_coordinates))
                .collect::<Vec<RawIntersect<S>>>()
                .into(),
        }
    }
}
