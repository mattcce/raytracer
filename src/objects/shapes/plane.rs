use crate::collections::{Point, Vector};
use crate::objects::*;
use crate::utils::{Buildable, ConsumingBuilder, EPSILON};

#[derive(Debug)]
pub struct Plane {
    frame_transformation: Transform,
    material: Material,
    bounds: Bounds,
}

impl Plane {
    const PRIMITIVE_BOUNDING_BOX: BoundingBox = BoundingBox::from_axial_bounds(
        [f64::NEG_INFINITY, f64::INFINITY],
        [0.0, 0.0],
        [f64::NEG_INFINITY, f64::INFINITY],
    );
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

impl Bounded for Plane {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

#[derive(Debug, Default)]
pub struct PlaneBuilder {
    frame_transformation: Option<Transform>,
    material: Option<Material>,
}

impl PlaneBuilder {
    pub fn set_frame_transformation(mut self, frame_transformation: Transform) -> PlaneBuilder {
        self.frame_transformation = Some(frame_transformation);
        self
    }

    pub fn set_material(mut self, material: Material) -> PlaneBuilder {
        self.material = Some(material);
        self
    }
}

impl Buildable for Plane {
    type Builder = PlaneBuilder;

    fn builder() -> Self::Builder {
        PlaneBuilder::default()
    }
}

impl ConsumingBuilder for PlaneBuilder {
    type Built = Plane;

    fn build(self) -> Self::Built {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let material = self.material.unwrap_or_default();
        let bounds = Bounds::new(Plane::PRIMITIVE_BOUNDING_BOX.transform(&frame_transformation));

        let plane = Plane {
            frame_transformation,
            material,
            bounds,
        };
        plane
    }
}

impl Into<Shape> for Plane {
    fn into(self) -> Shape {
        Shape::Primitive(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::{Point, Vector};
    use crate::utils::BuildInto;

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
        let default_plane: Shape = Plane::builder().build_into();
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }

    #[test]
    fn intersect_ray_coplanar_to_plane() {
        let default_plane: Shape = Plane::builder().build_into();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }

    #[test]
    fn intersect_plane_from_above() {
        let default_plane: Shape = Plane::builder().build_into();
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 1.0);
    }

    #[test]
    fn intersect_plane_from_below() {
        let default_plane: Shape = Plane::builder().build_into();
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let hit_register = default_plane.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 1.0);
    }
}
