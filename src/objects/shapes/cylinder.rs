use crate::collections::{Point, Vector};
use crate::objects::{Coordinates, Material, PrimitiveShape, Ray, Shape, ShapeBuilder, Transform};
use crate::utils::EPSILON;

#[derive(Debug)]
pub struct Cylinder {
    frame_transformation: Transform,
    material: Material,
    y_minimum: f64,
    closed_bot: bool,
    y_maximum: f64,
    closed_top: bool,
}

impl Cylinder {
    pub fn builder() -> ShapeBuilder<Cylinder> {
        ShapeBuilder::default()
    }

    pub fn y_minimum(&mut self) -> Option<f64> {
        if self.closed_bot {
            None
        } else {
            Some(self.y_minimum)
        }
    }

    pub fn y_maximum(&mut self) -> Option<f64> {
        if self.closed_bot {
            None
        } else {
            Some(self.y_maximum)
        }
    }

    fn intersect_walls(&self, local_ray: &Ray) -> Vec<f64> {
        let &Ray { origin, direction } = local_ray;
        let Point {
            x: origin_x,
            y: _origin_y,
            z: origin_z,
        } = origin;
        let Vector {
            x: dir_x,
            y: _dir_y,
            z: dir_z,
        } = direction;

        let a = dir_x.powi(2) + dir_z.powi(2);

        if a.abs() < EPSILON {
            return vec![];
        }

        let b = (2.0 * origin_x * dir_x) + (2.0 * origin_z * dir_z);
        let c = origin_x.powi(2) + origin_z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return vec![];
        }

        let mut t_values = vec![];

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let y0 = local_ray.position(t0).y;
        if (self.y_minimum < y0) && (y0 < self.y_maximum) {
            t_values.push(t0);
        }

        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let y1 = local_ray.position(t1).y;
        if (self.y_minimum < y1) && (y1 < self.y_maximum) {
            t_values.push(t1);
        }

        t_values
    }

    fn check_cap(local_ray: &Ray, t: f64) -> bool {
        let position = local_ray.position(t);

        (position.x.powi(2) + position.z.powi(2)) <= 1.0
    }

    fn intersect_caps(&self, local_ray: &Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let mut t_values = vec![];

        if self.closed_bot {
            let t = (self.y_minimum - local_ray.origin.y) / local_ray.direction.y;
            if Self::check_cap(local_ray, t) {
                t_values.push(t);
            }
        }

        if self.closed_top {
            let t = (self.y_maximum - local_ray.origin.y) / local_ray.direction.y;
            if Self::check_cap(local_ray, t) {
                t_values.push(t);
            }
        }

        t_values
    }
}

impl PrimitiveShape for Cylinder {
    fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, local_point: Point, _: Option<(f64, f64)>) -> Vector {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < 1.0 {
            match local_point.y {
                y if y >= self.y_maximum - EPSILON => return Vector::new(0.0, 1.0, 0.0),
                y if y <= self.y_minimum + EPSILON => return Vector::new(0.0, -1.0, 0.0),
                _ => (),
            }
        }

        Vector::new(local_point.x, 0.0, local_point.z)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<Coordinates> {
        let mut t_values = vec![];

        t_values.extend_from_slice(&self.intersect_walls(local_ray));
        t_values.extend_from_slice(&self.intersect_caps(local_ray));

        t_values
            .iter()
            .map(|&t| Coordinates::new(t, None))
            .collect()
    }
}

impl ShapeBuilder<Cylinder> {
    pub fn set_y_minimum(mut self, y_minimum: f64) -> ShapeBuilder<Cylinder> {
        self.y_minimum = Some(y_minimum);
        self
    }

    pub fn set_y_maximum(mut self, y_maximum: f64) -> ShapeBuilder<Cylinder> {
        self.y_maximum = Some(y_maximum);
        self
    }

    pub fn build(self) -> Cylinder {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let material = self.material.unwrap_or_default();
        let (y_minimum, closed_bot) = match self.y_minimum {
            Some(y_minimum) => (y_minimum, true),
            None => (f64::NEG_INFINITY, false),
        };
        let (y_maximum, closed_top) = match self.y_maximum {
            Some(y_maximum) => (y_maximum, true),
            None => (f64::INFINITY, false),
        };

        let cylinder = Cylinder {
            frame_transformation,
            material,
            y_minimum,
            closed_bot,
            y_maximum,
            closed_top,
        };
        cylinder
    }

    pub fn wrap(self) -> Shape {
        let cylinder = self.build();
        Shape::wrap_primitive(cylinder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::approx_eq;

    #[test]
    fn ray_misses_cylinder() {
        let cylinder = Cylinder::builder().build();
        let test_cases: [(Point, Vector); 3] = [
            (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
        ];
        for (origin, direction) in test_cases {
            let ray = Ray::new(origin, direction.normalise());
            assert_eq!(cylinder.local_intersect(&ray).len(), 0);
        }
    }

    #[test]
    fn ray_hits_cylinder() {
        let cylinder = Cylinder::builder().build();
        let test_cases: [(Point, Vector, f64, f64); 3] = [
            (
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.807982,
                7.088723,
            ),
        ];
        for (origin, direction, t0, t1) in test_cases {
            let ray = Ray::new(origin, direction.normalise());
            let t_values = cylinder.local_intersect(&ray);
            assert_eq!(t_values.len(), 2);
            approx_eq!(t_values[0].t(), t0);
            approx_eq!(t_values[1].t(), t1);
        }
    }

    #[test]
    fn normal_on_cylinder() {
        let cylinder = Cylinder::builder().build();
        let test_cases: [(Point, Vector); 4] = [
            (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, normal) in test_cases {
            assert_eq!(cylinder.local_normal_at(point, None), normal);
        }
    }

    #[test]
    fn intersect_ray_with_constrained_cylinder() {
        let cylinder = Cylinder::builder()
            .set_y_minimum(1.0)
            .set_y_maximum(2.0)
            .build();
        let test_cases: [(Point, Vector, usize); 5] = [
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];
        for (origin, direction, count) in test_cases {
            let ray = Ray::new(origin, direction.normalise());
            assert_eq!(cylinder.local_intersect(&ray).len(), count);
        }
    }

    #[test]
    fn intersect_caps_of_closed_cylinder() {
        let cylinder = Cylinder::builder()
            .set_y_minimum(1.0)
            .set_y_maximum(2.0)
            .build();
        let test_cases: [(Point, Vector, usize); 5] = [
            (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
            (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
            (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
            (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
        ];
        for (origin, direction, count) in test_cases {
            let ray = Ray::new(origin, direction.normalise());
            assert_eq!(cylinder.local_intersect(&ray).len(), count);
        }
    }

    #[test]
    fn normal_on_capped_cylinder() {
        let cylinder = Cylinder::builder()
            .set_y_minimum(1.0)
            .set_y_maximum(2.0)
            .build();
        let test_cases: [(Point, Vector); 6] = [
            (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
        ];
        for (point, normal) in test_cases {
            assert_eq!(cylinder.local_normal_at(point, None), normal);
        }
    }
}
