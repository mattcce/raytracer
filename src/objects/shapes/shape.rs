use std::fmt::Debug;

use crate::collections::{Point, Vector};
use crate::objects::*;

#[derive(Debug)]
pub enum Shape {
    Primitive(Box<dyn PrimitiveShape>),
    Group(Group),
    // CSG(Box<CSG>),
}

impl Intersectable<dyn PrimitiveShape> for Shape {
    fn intersect_ray<'world: 'ray, 'ray>(
        &'world self,
        world_ray: &'ray Ray,
        transform_stack: Vec<&'ray Transform>,
    ) -> HitRegister<'ray, dyn PrimitiveShape> {
        if !self.bounds().intersect_bounds(world_ray, &transform_stack) {
            return HitRegister::empty();
        }

        match self {
            Shape::Primitive(primitive) => primitive.intersect_ray(world_ray, transform_stack),
            Shape::Group(group) => group.intersect_ray(world_ray, transform_stack),
        }
    }
}

impl Bounded for Shape {
    fn bounds(&self) -> &Bounds {
        match self {
            Shape::Primitive(s) => s.bounds(),
            Shape::Group(s) => s.bounds(),
        }
    }
}

pub trait PrimitiveShape: Debug + Bounded {
    fn normal_at(
        &self,
        world_point: Point,
        uv_coordinates: Option<(f64, f64)>,
        transform_stack: &Vec<&Transform>,
    ) -> Vector {
        let local_point = transform_through_stack_forwards(world_point, &transform_stack);
        let local_normal = self.local_normal_at(local_point, uv_coordinates);
        let world_normal = transform_through_stack_backwards(local_normal, &transform_stack);
        world_normal.normalise()
    }

    fn frame_transformation(&self) -> &Transform;
    fn material(&self) -> &Material;
    fn local_normal_at(&self, local_point: Point, uv_coordinates: Option<(f64, f64)>) -> Vector;
    fn local_intersect(&self, local_ray: &Ray) -> Vec<Coordinates>;
}

impl PartialEq for dyn PrimitiveShape {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

pub trait Intersectable<S: PrimitiveShape + PartialEq + ?Sized> {
    fn intersect_ray<'a: 'r, 'r>(
        &'a self,
        world_ray: &'r Ray,
        transform_stack: Vec<&'r Transform>,
    ) -> HitRegister<'r, S>;
}

impl<S: PrimitiveShape + PartialEq + ?Sized> Intersectable<S> for S {
    fn intersect_ray<'a: 'r, 'r>(
        &'a self,
        world_ray: &'r Ray,
        mut transform_stack: Vec<&'r Transform>,
    ) -> HitRegister<'r, Self> {
        let mut hit_register = HitRegister::empty();
        transform_stack.push(self.frame_transformation());
        let local_ray = transform_through_stack_forwards(*world_ray, &transform_stack);
        let coordinates = self.local_intersect(&local_ray);

        for coordinate in coordinates {
            let raw_intersect = coordinate.attach(self, world_ray, transform_stack.clone());
            hit_register.add_raw_intersect(raw_intersect);
        }

        hit_register
    }
}

pub(crate) fn transform_through_stack_forwards<T: Transformable>(
    mut object: T,
    transform_stack: &Vec<&Transform>,
) -> T {
    for &transform in transform_stack {
        object = object.transform(&transform.invert());
    }

    object
}

pub(crate) fn transform_through_stack_backwards<T: Transformable>(
    mut object: T,
    transform_stack: &Vec<&Transform>,
) -> T {
    for &transform in transform_stack.iter().rev() {
        object = object.transform(&transform.invert().transpose());
    }

    object
}
