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
    inside: bool,
}

impl<'a> Comps<'a> {
    fn prepare(intersect: &'a Intersect<'a>, ray: Ray) -> Comps<'a> {
        let t = intersect.t();
        let object = intersect.object();
        let point = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = object.normal_at(point);
        let inside = match normalv.dot(eyev) {
            _x if _x < 0.0 => {
                normalv = -normalv;
                true
            }
            _x if _x >= 0.0 => false,
            _ => panic!(),
        };
        Comps {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Intersect, Ray};

    #[test]
    fn prepare_comps_ray_outside() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let intersect = Intersect::new(4.0, &shape);
        let resulting_comps = Comps {
            t: intersect.t(),
            object: intersect.object(),
            point: Point::new(0.0, 0.0, -1.0),
            eyev: Vector::new(0.0, 0.0, -1.0),
            normalv: Vector::new(0.0, 0.0, -1.0),
            inside: false,
        };
        assert_eq!(Comps::prepare(&intersect, ray), resulting_comps);
    }

    #[test]
    fn prepare_comps_ray_inside() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let intersect = Intersect::new(1.0, &shape);
        let prepared_comps = Comps::prepare(&intersect, ray);
        assert_eq!(prepared_comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(prepared_comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(prepared_comps.inside, true);
        assert_eq!(prepared_comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
}
