use std::marker::PhantomData;

use crate::collections::{Colour, Point, Vector};
use crate::objects::{PrimitiveShape, Transform};
use crate::utils::floats::EPSILON;

use super::Light;
use super::Ray;

pub struct Coordinates {
    t: f64,
    uv_coordinates: Option<(f64, f64)>,
}

impl Coordinates {
    pub fn new(t: f64, uv_coordinates: Option<(f64, f64)>) -> Coordinates {
        Coordinates { t, uv_coordinates }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn uv_coordinates(&self) -> Option<(f64, f64)> {
        self.uv_coordinates
    }

    pub fn attach<'ray, S>(
        self,
        object: &'ray S,
        ray: &'ray Ray,
        transform_stack: Vec<&'ray Transform>,
    ) -> Intersect<'ray, S, Raw>
    where
        S: PrimitiveShape + ?Sized,
    {
        let Coordinates { t, uv_coordinates } = self;
        Intersect::new(t, object, ray, uv_coordinates, transform_stack)
    }
}

mod private {
    pub trait Sealed {}
    impl<S: Sealed> super::IntersectState for S {}
    impl Sealed for super::Raw {}
    impl Sealed for super::Computed {}
}

pub trait IntersectState: private::Sealed {}

#[derive(Clone, Copy, Debug)]
pub struct Raw;

#[derive(Clone, Copy, Debug)]
pub struct Computed;

#[derive(Clone, Debug)]
pub struct Intersect<'ray, S, ItxState = Raw>
where
    S: PrimitiveShape + ?Sized,
    ItxState: IntersectState,
{
    state: PhantomData<ItxState>,
    t: f64,
    object: &'ray S,
    ray: &'ray Ray,
    uv_coordinates: Option<(f64, f64)>,
    transform_stack: Vec<&'ray Transform>,
    computations: Option<Box<Computations>>,
}

impl<'ray, S, ItxState> Intersect<'ray, S, ItxState>
where
    S: PrimitiveShape + ?Sized,
    ItxState: IntersectState,
{
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'ray S {
        self.object
    }

    pub fn ray(&self) -> &'ray Ray {
        self.ray
    }

    pub fn uv_coordinates(&self) -> Option<(f64, f64)> {
        self.uv_coordinates
    }

    pub fn transform_stack(&self) -> &Vec<&'ray Transform> {
        &self.transform_stack
    }
}

impl<'ray, S> Intersect<'ray, S, Raw>
where
    S: PrimitiveShape + ?Sized,
{
    pub fn new(
        t: f64,
        object: &'ray S,
        ray: &'ray Ray,
        uv_coordinates: Option<(f64, f64)>,
        transform_stack: Vec<&'ray Transform>,
    ) -> Intersect<'ray, S, Raw> {
        Intersect {
            state: PhantomData,
            t,
            object,
            ray,
            uv_coordinates,
            transform_stack,
            computations: None,
        }
    }

    fn compute(self, refraction_boundary: (f64, f64)) -> Intersect<'ray, S, Computed> {
        let Intersect {
            t,
            object,
            ray,
            uv_coordinates,
            transform_stack,
            ..
        } = self;
        let target = self.ray.position(t);
        let eyev = -self.ray.direction;
        let mut normal = object.normal_at(target, uv_coordinates, &transform_stack);
        let inside = match normal.dot(eyev) {
            _x if _x < 0.0 => {
                normal = -normal;
                true
            }
            _x if _x >= 0.0 => false,
            _ => panic!(),
        };
        let over_point = target + normal * EPSILON;
        let under_point = target - normal * EPSILON;
        let reflected_ray = Ray::new(over_point, ray.direction.reflect(normal));

        let computations = Some(Box::new(Computations {
            target,
            eyev,
            normal,
            inside,
            over_point,
            under_point,
            reflected_ray,
            refraction_boundary,
        }));
        Intersect {
            state: PhantomData,
            t,
            object,
            ray,
            uv_coordinates,
            transform_stack,
            computations,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Computations {
    target: Point,
    eyev: Vector,
    normal: Vector,
    inside: bool,
    over_point: Point,
    under_point: Point,
    reflected_ray: Ray,
    refraction_boundary: (f64, f64),
}

impl Computations {
    pub fn target(&self) -> Point {
        self.target
    }

    pub fn eyev(&self) -> Vector {
        self.eyev
    }

    pub fn normal(&self) -> Vector {
        self.normal
    }

    pub fn inside(&self) -> bool {
        self.inside
    }

    pub fn over_point(&self) -> Point {
        self.over_point
    }

    pub fn under_point(&self) -> Point {
        self.under_point
    }

    pub fn reflected_ray(&self) -> Ray {
        self.reflected_ray
    }

    pub fn refraction_boundary(&self) -> (f64, f64) {
        self.refraction_boundary
    }
}

impl<'ray, S> Intersect<'ray, S, Computed>
where
    S: PrimitiveShape + ?Sized,
{
    fn computations(&self) -> &Computations {
        self.computations.as_ref().unwrap()
    }

    pub fn target(&self) -> Point {
        self.computations().target()
    }

    pub fn eyev(&self) -> Vector {
        self.computations().eyev()
    }

    pub fn normal(&self) -> Vector {
        self.computations().normal()
    }

    pub fn inside(&self) -> bool {
        self.computations().inside()
    }

    pub fn over_point(&self) -> Point {
        self.computations().over_point()
    }

    pub fn under_point(&self) -> Point {
        self.computations().under_point()
    }

    pub fn reflected_ray(&self) -> Ray {
        self.computations().reflected_ray()
    }

    pub fn refraction_boundary(&self) -> (f64, f64) {
        self.computations().refraction_boundary()
    }

    pub(crate) fn shade(&self, light: &Light, shadowed: bool) -> Colour {
        light.shade_phong(
            self.object().material(),
            self.over_point(),
            self.eyev(),
            self.normal(),
            shadowed,
        )
    }

    pub(crate) fn schlick_reflectance(&self) -> f64 {
        let (n1, n2) = self.refraction_boundary();
        let mut cos = self.eyev().dot(self.normal());

        if n1 > n2 {
            let n = n1 / n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();

            cos = cos_t
        }

        let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[derive(Clone, Debug)]
pub struct HitRegister<'ray, S>(Vec<Intersect<'ray, S, Raw>>)
where
    S: PrimitiveShape + ?Sized + PartialEq;

impl<'ray, S> HitRegister<'ray, S>
where
    S: PrimitiveShape + ?Sized + PartialEq,
{
    pub fn empty() -> HitRegister<'ray, S> {
        HitRegister(vec![])
    }

    pub fn add_raw_intersect(&mut self, intersect: Intersect<'ray, S>) {
        self.0.push(intersect);
    }

    pub fn combine_registers(&mut self, mut hit_register: HitRegister<'ray, S>) {
        self.0.append(&mut hit_register.0);
    }

    pub fn finalise_hit(mut self) -> Option<Intersect<'ray, S, Computed>> {
        self.sort_intersections_by_t();
        match self.0.iter().position(|itx| itx.t >= 0.0) {
            Some(idx_hit) => {
                let refraction_boundary = self.compute_refraction_boundary(idx_hit);
                Some(self.0.swap_remove(idx_hit).compute(refraction_boundary))
            }
            None => None,
        }
    }

    fn sort_intersections_by_t(&mut self) {
        self.0.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
    }

    fn compute_refraction_boundary(&self, idx_hit: usize) -> (f64, f64) {
        assert!(idx_hit < self.0.len());

        let mut in_objects: Vec<&S> = vec![];

        for (idx_current_intersect, current_intersect) in self.0.iter().enumerate() {
            if idx_current_intersect == idx_hit {
                let n1 = match in_objects.last() {
                    Some(last_object) => last_object.material().refractive_index,
                    None => 1.0,
                };

                HitRegister::update_containers(&mut in_objects, current_intersect);

                let n2 = match in_objects.last() {
                    Some(last_object) => last_object.material().refractive_index,
                    None => 1.0,
                };

                return (n1, n2);
            } else {
                HitRegister::update_containers(&mut in_objects, current_intersect);
            }
        }

        panic!();
    }

    fn update_containers<'tmp>(
        in_objects: &mut Vec<&'tmp S>,
        current_intersect: &Intersect<'ray, S>,
    ) where
        'ray: 'tmp,
    {
        match in_objects
            .iter()
            .position(|&object| object == current_intersect.object)
        {
            Some(idx_object) => {
                in_objects.remove(idx_object);
            }
            None => {
                in_objects.push(current_intersect.object);
            }
        };
    }
}

impl<'ray, S> From<Vec<Intersect<'ray, S>>> for HitRegister<'ray, S>
where
    S: PrimitiveShape + ?Sized + PartialEq,
{
    fn from(value: Vec<Intersect<'ray, S>>) -> HitRegister<'ray, S> {
        HitRegister(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Material, Plane, Sphere, Transform, TransformKind};
    use crate::scenes::World;
    use crate::utils::{BuildInto, Buildable, ConsumingBuilder};

    #[test]
    fn compute_intersect_ray_outside() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::builder().build();
        let raw_intersect = Intersect::new(4.0, &shape, &ray, None, vec![]);
        let computed_intersect = raw_intersect.compute((0.0, 0.0));
        assert_eq!(computed_intersect.target(), Point::new(0.0, 0.0, -1.0));
        assert_eq!(computed_intersect.eyev(), Vector::new(0.0, 0.0, -1.0));
        assert_eq!(computed_intersect.normal(), Vector::new(0.0, 0.0, -1.0));
        assert_eq!(
            computed_intersect.over_point(),
            Point::new(0.0, 0.0, -1.0) + Vector::new(0.0, 0.0, -1.0) * EPSILON
        );
    }

    #[test]
    fn hit_offset_point() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, 1.0)))
            .set_material(Material::preset())
            .build();
        let transform = Transform::new(TransformKind::Translate(0.0, 0.0, 1.0));
        let raw_intersect = Intersect::new(5.0, &shape, &ray, None, vec![&transform]);
        let computed_intersect = raw_intersect.compute((0.0, 0.0));
        assert!(computed_intersect.over_point().z < -EPSILON / 2.0);
        assert!(computed_intersect.target().z > computed_intersect.over_point().z);
        assert!(computed_intersect.under_point().z > -EPSILON / 2.0);
        assert!(computed_intersect.target().z < computed_intersect.under_point().z);
    }

    #[test]
    fn compute_intersect_ray_inside() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::builder().build();
        let raw_intersect = Intersect::new(1.0, &shape, &ray, None, vec![]);
        let computed_intersect = raw_intersect.compute((0.0, 0.0));
        assert_eq!(computed_intersect.target(), Point::new(0.0, 0.0, 1.0));
        assert_eq!(computed_intersect.eyev(), Vector::new(0.0, 0.0, -1.0));
        assert!(computed_intersect.inside());
        assert_eq!(computed_intersect.normal(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn precompute_reflection_vector() {
        let plane = Plane::builder().set_material(Material::preset()).build();
        let ray = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let raw_intersect = Intersect::new(2.0_f64.sqrt() / 2.0, &plane, &ray, None, vec![]);
        let computed_intersect = raw_intersect.compute((0.0, 0.0));
        assert_eq!(
            computed_intersect.reflected_ray().direction,
            Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn hit_register_finalises_hit() {
        let sphere = Sphere::builder().build();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let intersect1 = Intersect::new(-1.0, &sphere, &ray, None, vec![]);
        let intersect2 = Intersect::new(2.0, &sphere, &ray, None, vec![]);
        let intersect3 = Intersect::new(3.0, &sphere, &ray, None, vec![]);
        let hit_register = HitRegister::from(vec![intersect1, intersect2, intersect3]);
        let hit = hit_register.finalise_hit().unwrap();
        assert_eq!(hit.t(), 2.0);
    }

    #[test]
    fn refractive_indices_at_various_intersections() {
        let s1 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)))
            .set_material(Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Material::preset()
            })
            .build_into();

        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, -0.25)))
            .set_material(Material {
                transparency: 1.0,
                refractive_index: 2.0,
                ..Material::preset()
            })
            .build_into();
        let s3 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, 0.25)))
            .set_material(Material {
                transparency: 1.0,
                refractive_index: 2.5,
                ..Material::preset()
            })
            .build_into();
        let world = World::new(vec![s1, s2, s3], vec![]);
        let ray = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let mut hit_register = world.intersect_ray(&ray);
        hit_register.sort_intersections_by_t();

        let test_cases: [(usize, f64, f64); 6] = [
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];
        for (idx, n1, n2) in test_cases {
            let refraction_boundary = hit_register.compute_refraction_boundary(idx);
            assert_eq!(refraction_boundary, (n1, n2), "{}", idx);
        }
    }
}
