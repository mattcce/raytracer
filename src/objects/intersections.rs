use std::ops::Index;

use crate::collections::{Colour, Point, Vector};
use crate::objects::Shape;
use crate::utils::floats::EPSILON;

use super::Light;
use super::Ray;

#[derive(Clone, Debug)]
pub struct RawIntersect<'a, S>
    where
        S: Shape + ?Sized,
{
    pub t: f64,
    pub object: &'a S,
    pub ray: &'a Ray,
}

impl<'a, S> RawIntersect<'a, S>
    where
        S: Shape + ?Sized,
{
    pub fn new(t: f64, object: &'a S, ray: &'a Ray) -> RawIntersect<'a, S> {
        RawIntersect { t, object, ray }
    }

    pub fn precompute(&self) -> ComputedIntersect<'_, S> {
        let t = self.t;
        let object = self.object;
        let ray = self.ray;
        let target = self.ray.position(t);
        let eyev = -self.ray.direction;
        let mut normal = object.normal_at(target);
        let inside = match normal.dot(eyev) {
            _x if _x < 0.0 => {
                normal = -normal;
                true
            }
            _x if _x >= 0.0 => false,
            _ => panic!(),
        };
        let over_point = target + normal * EPSILON;
        ComputedIntersect {
            t,
            object,
            ray,
            target,
            eyev,
            normal,
            inside,
            over_point,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComputedIntersect<'a, S>
    where
        S: Shape + ?Sized,
{
    pub t: f64,
    pub object: &'a S,
    pub ray: &'a Ray,

    pub target: Point,
    pub eyev: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl<S> ComputedIntersect<'_, S>
    where
        S: Shape + ?Sized,
{
    pub fn shade(&self, light: &Light, shadowed: bool) -> Colour {
        light.shade_phong(
            self.object.material(),
            self.over_point,
            self.eyev,
            self.normal,
            shadowed,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Intersections<'a, S>(pub Vec<RawIntersect<'a, S>>)
    where
        S: Shape + ?Sized;

impl<'a, S> Intersections<'a, S>
    where
        S: Shape + ?Sized,
{
    pub fn new(mut vec: Vec<RawIntersect<'_, S>>) -> Intersections<'_, S> {
        assert_ne!(vec.len(), 0);
        vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections(vec)
    }

    pub fn add_raw_intersect(&mut self, intersect: RawIntersect<'a, S>) {
        for (i, v) in self.0.iter_mut().enumerate() {
            if intersect.t < v.t {
                self.0.insert(i, intersect);
                return;
            }
        }
        self.0.push(intersect);
    }

    pub fn combine_intersections(&mut self, intersections: Intersections<'a, S>) {
        for intersect in intersections.0 {
            self.add_raw_intersect(intersect);
        }
    }

    pub fn hit(&self) -> Option<&RawIntersect<'_, S>> {
        self.0.iter().find(|&v| v.t >= 0.0)
    }
}

impl<'a, S> From<Vec<RawIntersect<'a, S>>> for Intersections<'a, S>
    where
        S: Shape + ?Sized,
{
    fn from(value: Vec<RawIntersect<'a, S>>) -> Intersections<'a, S> {
        Intersections::new(value)
    }
}

impl<'a, S> Default for Intersections<'a, S>
    where
        S: Shape + ?Sized,
{
    fn default() -> Intersections<'a, S> {
        Intersections(vec![])
    }
}

impl<'a, S> Index<usize> for Intersections<'a, S>
    where
        S: Shape + ?Sized,
{
    type Output = RawIntersect<'a, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::{Sphere, Transform, TransformKind};
    use crate::utils::Preset;

    use super::*;

    #[test]
    fn create_raw_intersect() {
        let sphere = Sphere::default();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let raw_intersect = RawIntersect::new(1.0, &sphere, &ray);
        let resulting_intersect = RawIntersect {
            t: 1.0,
            object: &sphere,
            ray: &ray,
        };
        assert_eq!(raw_intersect.t, resulting_intersect.t);
        assert!(std::ptr::eq(
            raw_intersect.object,
            resulting_intersect.object,
        ));
        assert!(std::ptr::eq(raw_intersect.ray, resulting_intersect.ray));
    }

    #[test]
    fn compute_intersect_ray_outside() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let raw_intersect = RawIntersect::new(4.0, &shape, &ray);
        let computed_intersect = raw_intersect.precompute();
        assert_eq!(computed_intersect.target, Point::new(0.0, 0.0, -1.0));
        assert_eq!(computed_intersect.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(computed_intersect.normal, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(
            computed_intersect.over_point,
            Point::new(0.0, 0.0, -1.0) + Vector::new(0.0, 0.0, -1.0) * EPSILON
        );
    }

    #[test]
    fn compute_intersect_ray_inside() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let raw_intersect = RawIntersect::new(1.0, &shape, &ray);
        let computed_intersect = raw_intersect.precompute();
        assert_eq!(computed_intersect.target, Point::new(0.0, 0.0, 1.0));
        assert_eq!(computed_intersect.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(computed_intersect.inside);
        assert_eq!(computed_intersect.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn intersections_hit() {
        let sphere = Sphere::default();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let intersect1 = RawIntersect::new(-1.0, &sphere, &ray);
        let intersect2 = RawIntersect::new(2.0, &sphere, &ray);
        let intersect3 = RawIntersect::new(3.0, &sphere, &ray);
        let intersections = Intersections::new(vec![intersect1, intersect2, intersect3]);
        let resulting_hit = &intersections.0[1];
        assert!(std::ptr::eq(intersections.hit().unwrap(), resulting_hit));
    }

    #[test]
    fn hit_offset_point() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere {
            transform: Transform::new(TransformKind::Translate(0.0, 0.0, 1.0)),
            ..Sphere::preset()
        };
        let raw_intersect = RawIntersect::new(5.0, &shape, &ray);
        let computed_intersect = raw_intersect.precompute();
        assert!(computed_intersect.over_point.z < -EPSILON / 2.0);
        assert!(computed_intersect.target.z > computed_intersect.over_point.z);
    }
}
