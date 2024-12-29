use crate::collections::{Point, Vector};
use crate::objects::*;
use crate::utils::{Buildable, ConsumingBuilder, EPSILON};

#[derive(Debug)]
pub struct Cone {
    frame_transformation: Transform,
    material: Material,
    y_minimum: f64,
    closed_bot: bool,
    y_maximum: f64,
    closed_top: bool,
    bounds: Bounds,
}

impl Cone {
    const PRIMITIVE_BOUNDING_BOX: BoundingBox = BoundingBox::new_unbounded();

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
            y: origin_y,
            z: origin_z,
        } = origin;
        let Vector {
            x: dir_x,
            y: dir_y,
            z: dir_z,
        } = direction;

        let a = dir_x.powi(2) - dir_y.powi(2) + dir_z.powi(2);
        let b = 2.0 * origin_x * dir_x - 2.0 * origin_y * dir_y + 2.0 * origin_z * dir_z;
        let c = origin_x.powi(2) - origin_y.powi(2) + origin_z.powi(2);

        if a.abs() < EPSILON {
            return if b.abs() < EPSILON {
                vec![]
            } else {
                vec![-c / (2.0 * b)]
            };
        }

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

    fn intersect_caps(&self, local_ray: &Ray) -> Vec<f64> {
        fn check_cap(local_ray: &Ray, t: f64, y: f64) -> bool {
            let position = local_ray.position(t);

            (position.x.powi(2) + position.z.powi(2)) <= y.powi(2)
        }

        if local_ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let mut t_values = vec![];

        if self.closed_bot {
            let t = (self.y_minimum - local_ray.origin.y) / local_ray.direction.y;
            if check_cap(local_ray, t, self.y_minimum) {
                t_values.push(t);
            }
        }

        if self.closed_top {
            let t = (self.y_maximum - local_ray.origin.y) / local_ray.direction.y;
            if check_cap(local_ray, t, self.y_maximum) {
                t_values.push(t);
            }
        }

        t_values
    }
}

impl PrimitiveShape for Cone {
    fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, local_point: Point, _: Option<(f64, f64)>) -> Vector {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < f64::abs(local_point.y) {
            match local_point.y {
                y if y >= self.y_maximum - EPSILON => return Vector::new(0.0, 1.0, 0.0),
                y if y <= self.y_minimum + EPSILON => return Vector::new(0.0, -1.0, 0.0),
                _ => (),
            }
        }

        let y = match dist.sqrt() {
            y if local_point.y > 0.0 => -y,
            y if local_point.y <= 0.0 => y,
            _ => panic!(),
        };

        Vector::new(local_point.x, y, local_point.z)
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

impl Bounded for Cone {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

#[derive(Debug, Default)]
pub struct ConeBuilder {
    frame_transformation: Option<Transform>,
    material: Option<Material>,
    y_minimum: Option<f64>,
    y_maximum: Option<f64>,
}

impl ConeBuilder {
    pub fn set_frame_transformation(mut self, frame_transformation: Transform) -> ConeBuilder {
        self.frame_transformation = Some(frame_transformation);
        self
    }

    pub fn set_material(mut self, material: Material) -> ConeBuilder {
        self.material = Some(material);
        self
    }

    pub fn set_y_minimum(mut self, y_minimum: f64) -> ConeBuilder {
        self.y_minimum = Some(y_minimum);
        self
    }

    pub fn set_y_maximum(mut self, y_maximum: f64) -> ConeBuilder {
        self.y_maximum = Some(y_maximum);
        self
    }
}

impl Buildable for Cone {
    type Builder = ConeBuilder;

    fn builder() -> Self::Builder {
        ConeBuilder::default()
    }
}

impl ConsumingBuilder for ConeBuilder {
    type Built = Cone;

    fn build(self) -> Self::Built {
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
        let limit = f64::max(y_minimum.abs(), y_maximum.abs());
        let bounds = Bounds::new(
            Cone::PRIMITIVE_BOUNDING_BOX
                .bound_in_x_axis([-limit, limit])
                .bound_in_y_axis([y_minimum, y_maximum])
                .bound_in_z_axis([-limit, limit])
                .transform(&frame_transformation),
        );
        let cone = Cone {
            frame_transformation,
            material,
            y_minimum,
            closed_bot,
            y_maximum,
            closed_top,
            bounds,
        };
        cone
    }
}

impl Into<Shape> for Cone {
    fn into(self) -> Shape {
        Shape::Primitive(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::approx_eq;

    #[test]
    fn ray_intersects_cone() {
        let cone = Cone::builder().build();
        let test_cases: [(Point, Vector, f64, f64); 3] = [
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.660254,
                8.660254,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.550056,
                49.449944,
            ),
        ];
        for (origin, direction, t0, t1) in test_cases {
            let ray = Ray::new(origin, direction.normalise());
            let t_values = cone.local_intersect(&ray);
            approx_eq!(t_values[0].t(), t0);
            approx_eq!(t_values[1].t(), t1);
        }
    }

    #[test]
    fn ray_intersects_cone_parallel_to_one_half() {
        let cone = Cone::builder().build();
        let ray = Ray::new(
            Point::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 1.0).normalise(),
        );
        let t_values = cone.local_intersect(&ray);
        assert_eq!(t_values.len(), 1);
        approx_eq!(t_values[0].t(), 0.353553);
    }

    #[test]
    fn ray_intersects_caps() {
        let cone = Cone::builder()
            .set_y_minimum(-0.5)
            .set_y_maximum(0.5)
            .build();
        let test_cases: [(Point, Vector, usize); 3] = [
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];
        for (origin, direction, count) in test_cases {
            let ray = Ray::new(origin, direction.normalise());
            let t_values = cone.local_intersect(&ray);
            assert_eq!(t_values.len(), count);
        }
    }

    #[test]
    fn normal_vector_on_cone() {
        let cone = Cone::builder()
            .set_y_minimum(-0.5)
            .set_y_maximum(5.0)
            .build();
        let test_cases: [(Point, Vector); 3] = [
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (
                Point::new(1.0, 1.0, 1.0),
                Vector::new(1.0, -2.0_f64.sqrt(), 1.0),
            ),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];
        for (point, normal) in test_cases {
            assert_eq!(cone.local_normal_at(point, None), normal);
        }
    }

    #[test]
    fn primitive_cone_bounds() {
        let cone = Cone::builder().build();
        let bounds = cone.bounds();
        println!("{:?}", bounds);
        assert!(!bounds.bounding_box().is_bounded());
    }

    #[test]
    fn transformed_cone_bounds() {
        let cone = Cone::builder()
            .set_y_minimum(-5.0)
            .set_y_maximum(3.0)
            .build();
        let (x_range, y_range, z_range) = cone.bounds().bounding_box().axial_bounds();
        assert_eq!(x_range, [-5.0, 5.0]);
        assert_eq!(y_range, [-5.0, 3.0]);
        assert_eq!(z_range, [-5.0, 5.0]);
    }
}
