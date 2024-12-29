use crate::collections::{Point, Vector};
use crate::objects::{Coordinates, Material, PrimitiveShape, Ray, Shape, ShapeBuilder, Transform};
use crate::utils::EPSILON;

#[derive(Default, Debug)]
pub struct Plane {
    frame_transformation: Transform,
    material: Material,
}

impl Plane {
    pub fn builder() -> ShapeBuilder<Plane> {
        ShapeBuilder::default()
    }
}

impl PrimitiveShape for Plane {
    fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, _local_point: Point, _: Option<(f64, f64)>) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<Coordinates> {
        if local_ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let t = -local_ray.origin.y / local_ray.direction.y;
        vec![t].iter().map(|&t| Coordinates::new(t, None)).collect()
    }
}

impl ShapeBuilder<Plane> {
    pub fn build(self) -> Plane {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let material = self.material.unwrap_or_default();

        let plane = Plane {
            frame_transformation,
            material,
        };
        plane
    }

    pub fn wrap(self) -> Shape {
        let plane = self.build();
        Shape::wrap_primitive(plane)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::{Point, Vector};

    use super::*;

    #[test]
    fn normal_of_plane() {
        let default_plane = Plane::builder().build();
        let normal1 = default_plane.normal_at(Point::new(0.0, 0.0, 0.0), None, &vec![]);
        let normal2 = default_plane.normal_at(Point::new(10.0, 0.0, -10.0), None, &vec![]);
        let normal3 = default_plane.normal_at(Point::new(-5.0, 0.0, 150.0), None, &vec![]);
        let resulting_vector = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(normal1, resulting_vector);
        assert_eq!(normal2, resulting_vector);
        assert_eq!(normal3, resulting_vector);
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let default_plane = Plane::builder().wrap();
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }

    #[test]
    fn intersect_ray_coplanar_to_plane() {
        let default_plane = Plane::builder().wrap();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }

    #[test]
    fn intersect_plane_from_above() {
        let default_plane = Plane::builder().wrap();
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 1.0);
    }

    #[test]
    fn intersect_plane_from_below() {
        let default_plane = Plane::builder().wrap();
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 1.0);
    }
}
