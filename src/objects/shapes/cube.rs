use crate::collections::{Point, Vector};
use crate::objects::{Coordinates, Material, PrimitiveShape, Ray, Shape, ShapeBuilder, Transform};
use crate::utils::floats::EPSILON;

#[derive(Debug)]
pub struct Cube {
    frame_transformation: Transform,
    material: Material,
}

impl Cube {
    pub fn builder() -> ShapeBuilder<Cube> {
        ShapeBuilder::default()
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let tmin;
        let tmax;
        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl PrimitiveShape for Cube {
    fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, local_point: Point, _: Option<(f64, f64)>) -> Vector {
        let maxc = [
            local_point.x.abs(),
            local_point.y.abs(),
            local_point.z.abs(),
        ]
        .into_iter()
        .reduce(f64::max)
        .unwrap();

        match maxc {
            x if x == local_point.x.abs() => Vector::new(local_point.x, 0.0, 0.0),
            y if y == local_point.y.abs() => Vector::new(0.0, local_point.y, 0.0),
            z if z == local_point.z.abs() => Vector::new(0.0, 0.0, local_point.z),
            _ => panic!(),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<Coordinates> {
        let (xtmin, xtmax) = Cube::check_axis(local_ray.origin.x, local_ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(local_ray.origin.y, local_ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(local_ray.origin.z, local_ray.direction.z);

        let tmin = [xtmin, ytmin, ztmin].into_iter().reduce(f64::max).unwrap();
        let tmax = [xtmax, ytmax, ztmax].into_iter().reduce(f64::min).unwrap();

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
                .iter()
                .map(|&t| Coordinates::new(t, None))
                .collect()
        }
    }
}

impl ShapeBuilder<Cube> {
    pub fn build(self) -> Cube {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let material = self.material.unwrap_or_default();

        let cube = Cube {
            frame_transformation,
            material,
        };
        cube
    }

    pub fn wrap(self) -> Shape {
        let cube = self.build();
        Shape::wrap_primitive(cube)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::{Point, Vector};

    #[test]
    fn ray_intersects_cube() {
        let cube = Cube::builder().build();
        let test_cases: [(Point, Vector, f64, f64); 7] = [
            (
                Point::new(5.0, 0.5, 0.0),
                Vector::new(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(-5.0, 0.5, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 5.0, 0.0),
                Vector::new(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, -5.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, 5.0),
                Vector::new(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.0, 0.5, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];
        for (origin, direction, t1, t2) in test_cases {
            let ray = Ray::new(origin, direction);
            let t_values = cube.local_intersect(&ray);
            assert_eq!(t_values.len(), 2);
            assert_eq!(t_values[0].t(), t1);
            assert_eq!(t_values[1].t(), t2);
        }
    }

    #[test]
    fn ray_does_not_intersect_cube() {
        let cube = Cube::builder().build();
        let test_cases: [(Point, Vector); 6] = [
            (
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018),
            ),
            (
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345),
            ),
            (
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673),
            ),
            (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (origin, direction) in test_cases {
            let ray = Ray::new(origin, direction);
            assert_eq!(cube.local_intersect(&ray).len(), 0);
        }
    }

    #[test]
    fn normal_on_cube() {
        let cube = Cube::builder().build();
        let test_cases: [(Point, Vector); 8] = [
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, normal) in test_cases {
            assert_eq!(cube.local_normal_at(point, None), normal);
        }
    }
}
