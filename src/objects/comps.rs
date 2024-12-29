use super::Intersect;
use super::Ray;
use super::Sphere;
use crate::collections::{Point, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Comps<'a> {
    t: f64,
    object: &'a Sphere,
    point: Point,
    eyev: Vector,
    normalv: Vector,
}

impl<'a> Comps<'a> {
    fn prepare(intersect: &'a Intersect<'a>, ray: Ray) -> Comps<'a> {
        let t = intersect.t();
        let object = intersect.object();
        let point = ray.position(t);
        let eyev = -ray.direction;
        let normalv = object.normal_at(point);
        Comps {
            t,
            object,
            point,
            eyev,
            normalv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Intersect, Ray};

    #[test]
    fn precompute_state() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let intersect = Intersect::new(4.0, &shape);
        let resulting_comps = Comps {
            t: intersect.t(),
            object: intersect.object(),
            point: Point::new(0.0, 0.0, -1.0),
            eyev: Vector::new(0.0, 0.0, -1.0),
            normalv: Vector::new(0.0, 0.0, -1.0),
        };
        assert_eq!(Comps::prepare(&intersect, ray), resulting_comps);
    }
}
