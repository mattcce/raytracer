use std::fmt::Debug;
use std::marker::PhantomData;

use crate::collections::{Point, Vector};
use crate::objects::*;

#[derive(Debug)]
pub enum Shape {
    Primitive(Box<dyn PrimitiveShape>),
    Group(Group),
    // CSG(Box<CSG>),
}

impl Shape {
    pub fn wrap_primitive<S>(object: S) -> Shape
    where
        // here, PrimitiveShapes own everything and thus do not own references that outlive itself
        S: 'static + PrimitiveShape,
    {
        let primitive_shape: Box<dyn PrimitiveShape> = Box::new(object);
        Shape::Primitive(primitive_shape)
    }

    pub fn wrap_group(object: Group) -> Shape {
        Shape::Group(object)
    }

    pub fn intersect_ray<'world: 'ray, 'ray>(
        &'world self,
        ray: &'ray Ray,
        transform_stack: Vec<&'ray Transform>,
    ) -> HitRegister<'ray, dyn PrimitiveShape> {
        match self {
            Shape::Primitive(primitive) => primitive.intersect_ray(ray, transform_stack),
            Shape::Group(group) => group.intersect_ray(ray, transform_stack),
        }
    }
}

pub trait PrimitiveShape: Debug {
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

fn transform_through_stack_forwards<T: Transformable>(
    mut object: T,
    transform_stack: &Vec<&Transform>,
) -> T {
    for &transform in transform_stack {
        object = object.transform(&transform.invert());
    }

    object
}

fn transform_through_stack_backwards<T: Transformable>(
    mut object: T,
    transform_stack: &Vec<&Transform>,
) -> T {
    for &transform in transform_stack.iter().rev() {
        object = object.transform(&transform.invert().transpose());
    }

    object
}

mod private {
    pub trait Sealed {}
    impl<S: Sealed> super::BuildableShape for S {}
    impl Sealed for crate::objects::shapes::Cone {}
    impl Sealed for crate::objects::shapes::Cube {}
    impl Sealed for crate::objects::shapes::Plane {}
    impl Sealed for crate::objects::shapes::Sphere {}
    impl Sealed for crate::objects::shapes::Cylinder {}
    impl Sealed for crate::objects::shapes::Triangle {}
    impl Sealed for crate::objects::shapes::SmoothTriangle {}
    impl Sealed for crate::objects::group::Group {}
}
pub trait BuildableShape {}

#[derive(Debug)]
pub struct ShapeBuilder<Shp: BuildableShape> {
    pub(crate) shape: PhantomData<Shp>,
    pub(crate) frame_transformation: Option<Transform>,
    pub(crate) material: Option<Material>,
    pub(crate) y_minimum: Option<f64>,
    pub(crate) y_maximum: Option<f64>,
    pub(crate) vertices: Option<[Point; 3]>,
    pub(crate) normals: Option<[Vector; 3]>,
    pub(crate) objects: Option<Vec<Shape>>,
}

impl<Shp: BuildableShape> ShapeBuilder<Shp> {
    pub fn set_frame_transformation(
        mut self,
        frame_transformation: Transform,
    ) -> ShapeBuilder<Shp> {
        self.frame_transformation = Some(frame_transformation);
        self
    }
}

impl<Shp: BuildableShape + PrimitiveShape> ShapeBuilder<Shp> {
    pub fn set_material(mut self, material: Material) -> ShapeBuilder<Shp> {
        self.material = Some(material);
        self
    }
}

impl<Shp: BuildableShape> Default for ShapeBuilder<Shp> {
    fn default() -> ShapeBuilder<Shp> {
        ShapeBuilder {
            shape: PhantomData,
            frame_transformation: None,
            material: None,
            y_minimum: None,
            y_maximum: None,
            vertices: None,
            normals: None,
            objects: None,
        }
    }
}
