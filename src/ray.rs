use crate::collections::{Point, Vector};
use crate::intersections::{Intersect, Intersections};
use crate::transform::{Transform, TransformKind, Transformable};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: Transform,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Transform::new(TransformKind::Identity),
        }
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn intersect<'a>(&'a self, s: &'a Sphere) -> Option<Intersections<'a>> {
        let transformed_ray = self.transform(&s.get_transform().invert());
        let sphere_to_ray = transformed_ray.origin - Point::zero();
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        let sqrt_discriminant = discriminant.sqrt();

        if sqrt_discriminant.is_nan() {
            None
        } else {
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            Some(Intersections::from(vec![
                Intersect::new(t1, s),
                Intersect::new(t2, s),
            ]))
        }
    }
}

impl Transformable for Ray {
    fn transform(self, transform: &Transform) -> Self {
        Ray::new(
            self.origin.transform(transform),
            self.direction.transform(transform),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sphere() {
        let sphere = Sphere::new();
        let resulting_sphere = Sphere {
            transform: Transform::new(TransformKind::Identity),
        };
        assert_eq!(sphere, resulting_sphere);
    }

    #[test]
    fn transform_sphere() {
        let mut sphere = Sphere::new();
        let transform = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        sphere.set_transform(&transform);
    }

    #[test]
    fn create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(6.0, 5.0, 4.0);
        let ray = Ray::new(origin, direction);
        let resulting_ray = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(6.0, 5.0, 4.0),
        };
        assert_eq!(ray, resulting_ray);
    }

    #[test]
    fn ray_position() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = ray.intersect(&sphere).unwrap();
        assert_eq!(intersections[0].t(), 4.0);
        assert_eq!(intersections[1].t(), 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = ray.intersect(&sphere).unwrap();
        assert_eq!(intersections[0].t(), 5.0);
        assert_eq!(intersections[1].t(), 5.0);
    }

    #[test]
    fn ray_does_not_intersect_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = ray.intersect(&sphere);
        assert_eq!(intersections, None);
    }

    #[test]
    fn ray_originates_within_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = ray.intersect(&sphere).unwrap();
        assert_eq!(intersections[0].t(), -1.0);
        assert_eq!(intersections[1].t(), 1.0);
    }

    #[test]
    fn ray_originates_after_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = ray.intersect(&sphere).unwrap();
        assert_eq!(intersections[0].t(), -6.0);
        assert_eq!(intersections[1].t(), -4.0);
    }

    #[test]
    fn ray_intersects_transformed_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Sphere::new();
        sphere.set_transform(&Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)));
        let intersections = ray.intersect(&sphere).unwrap();
        assert_eq!(intersections[0].t(), 3.0);
        assert_eq!(intersections[1].t(), 7.0);
    }

    #[test]
    fn ray_does_not_intersect_transformed_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Sphere::new();
        sphere.set_transform(&Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)));
        let intersections = ray.intersect(&sphere);
        assert_eq!(intersections, None);
    }
}