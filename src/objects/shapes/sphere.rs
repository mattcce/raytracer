use crate::collections::{Point, Vector};
use crate::objects::*;
use crate::utils::{Buildable, ConsumingBuilder, EPSILON};

#[derive(Default, Debug, PartialEq)]
pub struct Sphere {
    frame_transformation: Transform,
    material: Material,
}

impl PrimitiveShape for Sphere {
    fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, local_point: Point, _: Option<(f64, f64)>) -> Vector {
        local_point - Point::new(0.0, 0.0, 0.0)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<Coordinates> {
        let sphere_to_ray = local_ray.origin - Point::zero();
        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            vec![t1, t2]
                .iter()
                .map(|&t| Coordinates::new(t, None))
                .collect()
        }
    }
}

#[derive(Debug, Default)]
pub struct SphereBuilder {
    frame_transformation: Option<Transform>,
    material: Option<Material>,
}

impl SphereBuilder {
    pub fn set_frame_transformation(mut self, frame_transformation: Transform) -> SphereBuilder {
        self.frame_transformation = Some(frame_transformation);
        self
    }

    pub fn set_material(mut self, material: Material) -> SphereBuilder {
        self.material = Some(material);
        self
    }
}

impl Buildable for Sphere {
    type Builder = SphereBuilder;

    fn builder() -> Self::Builder {
        SphereBuilder::default()
    }
}

impl ConsumingBuilder for SphereBuilder {
    type Built = Sphere;

    fn build(self) -> Self::Built {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let material = self.material.unwrap_or_default();

        let sphere = Sphere {
            frame_transformation,
            material,
        };
        sphere
    }
}

impl Into<Shape> for Sphere {
    fn into(self) -> Shape {
        Shape::Primitive(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::Angle;
    use crate::objects::Axis;
    use crate::utils::approx_eq;

    #[test]
    fn normal_on_unit_sphere() {
        let sphere = Sphere::default();
        let point1 = Point::new(1.0, 0.0, 0.0);
        let point2 = Point::new(0.0, 1.0, 0.0);
        let point3 = Point::new(0.0, 0.0, 1.0);
        let point4 = Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        let normal1 = Vector::new(1.0, 0.0, 0.0);
        let normal2 = Vector::new(0.0, 1.0, 0.0);
        let normal3 = Vector::new(0.0, 0.0, 1.0);
        let normal4 = Vector::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(sphere.normal_at(point1, None, &vec![]), normal1);
        assert_eq!(sphere.normal_at(point2, None, &vec![]), normal2);
        assert_eq!(sphere.normal_at(point3, None, &vec![]), normal3);
        assert_eq!(sphere.normal_at(point4, None, &vec![]), normal4);
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let transform1 = Transform::new(TransformKind::Translate(0.0, 1.0, 0.0));
        let transform2 = Transform::from(vec![
            TransformKind::Rotate(Axis::Z, Angle::from_radians(std::f64::consts::PI / 5.0)),
            TransformKind::Scale(1.0, 0.5, 1.0),
        ]);
        let sphere1 = Sphere::builder()
            .set_frame_transformation(transform1)
            .build();
        let sphere2 = Sphere::builder()
            .set_frame_transformation(transform2)
            .build();
        let point1 = Point::new(0.0, 1.0 + 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let point2 = Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal1 = sphere1.normal_at(point1, None, &vec![sphere1.frame_transformation()]);
        let normal2 = sphere1.normal_at(point2, None, &vec![sphere2.frame_transformation()]);
        let resulting_normal1 = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let resulting_normal2 = Vector::new(0.0, 0.970143, -0.242535);
        approx_eq!(normal1.x, resulting_normal1.x);
        approx_eq!(normal1.y, resulting_normal1.y);
        approx_eq!(normal1.z, resulting_normal1.z);
        approx_eq!(normal2.x, resulting_normal2.x);
        approx_eq!(normal2.y, resulting_normal2.y);
        approx_eq!(normal2.z, resulting_normal2.z);
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::builder().build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 4.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::builder().build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 5.0);
    }

    #[test]
    fn ray_does_not_intersect_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::builder().build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }

    #[test]
    fn ray_originates_within_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::builder().build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 1.0);
    }

    #[test]
    fn ray_originates_after_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::builder().build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }

    #[test]
    fn ray_intersects_transformed_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let transform = Transform::new(TransformKind::Scale(2.0, 2.0, 2.0));
        let sphere = Sphere::builder()
            .set_frame_transformation(transform)
            .build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert_eq!(hit_register.finalise_hit().unwrap().t(), 3.0);
    }

    #[test]
    fn ray_does_not_intersect_transformed_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let transform = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        let sphere = Sphere::builder()
            .set_frame_transformation(transform)
            .build();
        let hit_register = sphere.intersect_ray(&ray, vec![]);
        assert!(hit_register.finalise_hit().is_none());
    }
}
