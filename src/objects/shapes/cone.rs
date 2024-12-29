use crate::collections::{Point, Vector};
use crate::objects::{Material, Ray, Shape, Transform};
use crate::utils::EPSILON;

#[derive(Debug)]
pub struct Cone {
    pub transform: Transform,
    pub material: Material,
    y_minimum: f64,
    closed_bot: bool,
    y_maximum: f64,
    closed_top: bool,
}

impl Cone {
    pub fn new(transform: Transform, material: Material, y_minimum: f64, y_maximum: f64) -> Self {
        let closed_bot = y_minimum > f64::NEG_INFINITY;
        let closed_top = y_maximum < f64::INFINITY;

        Self {
            transform,
            material,
            y_minimum,
            closed_bot,
            y_maximum,
            closed_top,
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
            if b.abs() < EPSILON {
                return vec![];
            } else {
                return vec![-c / (2.0 * b)];
            }
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

    fn check_cap(local_ray: &Ray, t: f64, y: f64) -> bool {
        let position = local_ray.position(t);

        (position.x.powi(2) + position.z.powi(2)) <= y.powi(2)
    }

    fn intersect_caps(&self, local_ray: &Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let mut t_values = vec![];

        if self.closed_bot {
            let t = (self.y_minimum - local_ray.origin.y) / local_ray.direction.y;
            if Self::check_cap(local_ray, t, self.y_minimum) {
                t_values.push(t);
            }
        }

        if self.closed_top {
            let t = (self.y_maximum - local_ray.origin.y) / local_ray.direction.y;
            if Self::check_cap(local_ray, t, self.y_maximum) {
                t_values.push(t);
            }
        }

        t_values
    }
}

impl Shape for Cone {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn transformation_matrix_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < 1.0 {
            match local_point.y {
                y if y > self.y_maximum - EPSILON => return Vector::new(0.0, 1.0, 0.0),
                y if y < self.y_minimum + EPSILON => return Vector::new(0.0, -1.0, 0.0),
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

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let mut t_values = vec![];

        t_values.extend_from_slice(&self.intersect_walls(local_ray));
        t_values.extend_from_slice(&self.intersect_caps(local_ray));

        t_values
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            material: Material::default(),
            y_maximum: f64::INFINITY,
            closed_top: false,
            y_minimum: f64::NEG_INFINITY,
            closed_bot: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn ray_intersects_cone() {
    //     let cone = Cone::default();
    //     let test_cases: [(Point, Vector, f64, f64); 3] = [
    //         (
    //             Point::new(0.0, 0.0, -5.0),
    //             Vector::new(0.0, 0.0, 1.0),
    //             5.0,
    //             5.0,
    //         ),
    //         (
    //             Point::new(0.0, 0.0, -5.0),
    //             Vector::new(1.0, 1.0, 1.0),
    //             8.66025,
    //             8.66025,
    //         ),
    //         (
    //             Point::new(1.0, 1.0, -5.0),
    //             Vector::new(-0.5, -1.0, 1.0),
    //             4.55006,
    //             49.44994,
    //         ),
    //     ];
    //     for (origin, direction, t0, t1) in test_cases {
    //         let ray = Ray::new(origin, direction.normalise());
    //         let t_values = cone.local_intersect(&ray);
    //         assert_eq!(t_values[0], t0);
    //         assert_eq!(t_values[1], t1);
    //     }
    // }

    // #[test]
    // fn ray_intersects_cone_parallel_to_one_half() {
    //     let cone = Cone::default();
    //     let ray = Ray::new(
    //         Point::new(0.0, 0.0, -1.0),
    //         Vector::new(0.0, 1.0, 1.0).normalise(),
    //     );
    //     let t_values = cone.local_intersect(&ray);
    //     assert_eq!(t_values.len(), 1);
    //     assert_eq!(t_values[0], 0.35355);
    // }

    #[test]
    fn ray_intersects_caps() {
        let cone = Cone::new(Transform::default(), Material::default(), -0.5, 0.5);
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
        let cone = Cone::new(Transform::default(), Material::default(), -0.5, 5.0);
        let test_cases: [(Point, Vector); 3] = [
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (
                Point::new(1.0, 1.0, 1.0),
                Vector::new(1.0, -2.0_f64.sqrt(), 1.0),
            ),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];
        for (point, normal) in test_cases {
            assert_eq!(cone.local_normal_at(point), normal);
        }
    }
}
