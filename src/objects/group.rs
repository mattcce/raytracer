use crate::objects::*;

#[derive(Default, Debug)]
pub struct Group {
    frame_transformation: Transform,
    objects: Vec<Shape>,
}

impl Group {
    pub fn builder() -> ShapeBuilder<Group> {
        ShapeBuilder::default()
    }

    pub fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    pub fn objects(&self) -> &Vec<Shape> {
        &self.objects
    }
}

impl Intersectable<dyn PrimitiveShape> for Group {
    fn intersect_ray<'world: 'ray, 'ray>(
        &'world self,
        world_ray: &'ray Ray,
        mut transform_stack: Vec<&'ray Transform>,
    ) -> HitRegister<'ray, dyn PrimitiveShape> {
        let mut ray_hit_register = HitRegister::empty();
        transform_stack.push(self.frame_transformation());

        for shape in &self.objects {
            match shape {
                Shape::Primitive(primitive_shape) => {
                    let shape_hit_register =
                        primitive_shape.intersect_ray(world_ray, transform_stack.clone());
                    ray_hit_register.combine_registers(shape_hit_register);
                }
                Shape::Group(group) => {
                    let shape_hit_register =
                        group.intersect_ray(world_ray, transform_stack.clone());
                    ray_hit_register.combine_registers(shape_hit_register);
                }
            }
        }

        ray_hit_register
    }
}

impl ShapeBuilder<Group> {
    pub fn set_objects(mut self, objects: Vec<Shape>) -> ShapeBuilder<Group> {
        self.objects = Some(objects);
        self
    }

    pub fn add_object(mut self, object: Shape) -> ShapeBuilder<Group> {
        match self.objects {
            Some(ref mut objects) => {
                objects.push(object);
            }
            None => self.objects = Some(vec![object]),
        }
        self
    }

    pub fn build(self) -> Group {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let objects = self.objects.unwrap_or_default();
        let group = Group {
            frame_transformation,
            objects,
        };
        group
    }

    pub fn wrap(self) -> Shape {
        let group = self.build();
        Shape::wrap_group(group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::{Angle, Point, Vector};
    use crate::objects::{Axis, Ray, Sphere, TransformKind};

    #[test]
    fn intersect_ray_with_nonempty_group() {
        let s1 = Sphere::builder().wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, -3.0)))
            .wrap();
        let s3 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)))
            .wrap();
        let objects = vec![s1, s2, s3];
        let group = Group::builder().set_objects(objects).wrap();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let shape = group
            .intersect_ray(&ray, vec![])
            .finalise_hit()
            .unwrap()
            .object();
        let resulting_shape = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, -3.0)))
            .build();
        assert_eq!(shape, &resulting_shape as &dyn PrimitiveShape);
    }

    #[test]
    fn intersect_transformed_group() {
        let s1 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)))
            .wrap();
        let objects = vec![s1];
        let group = Group::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)))
            .set_objects(objects)
            .wrap();
        let ray = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));

        let shape = group
            .intersect_ray(&ray, vec![])
            .finalise_hit()
            .unwrap()
            .object();
        let resulting_shape = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)))
            .build();
        assert_eq!(shape, &resulting_shape as &dyn PrimitiveShape);
    }

    #[test]
    fn transform_stack_propagates_through_groups() {
        let s1 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(5.0, 0.0, 0.0)))
            .wrap();
        let objects = vec![s1];

        let g2 = Group::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)))
            .set_objects(objects)
            .wrap();
        let g1 = Group::builder()
            .set_frame_transformation(Transform::new(TransformKind::Rotate(
                Axis::Y,
                Angle::from_radians(std::f64::consts::FRAC_PI_2),
            )))
            .set_objects(vec![g2])
            .build();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -1.0));

        let computed_intersect = g1.intersect_ray(&ray, vec![]).finalise_hit().unwrap();
        let transform_stack = computed_intersect.transform_stack();
        let t1 = Transform::new(TransformKind::Rotate(
            Axis::Y,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
        ));
        let t2 = Transform::new(TransformKind::Scale(2.0, 2.0, 2.0));
        let t3 = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        let resulting_transform_stack = vec![&t1, &t2, &t3];

        assert_eq!(transform_stack, &resulting_transform_stack);
    }
}
