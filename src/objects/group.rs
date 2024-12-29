use std::cell::RefCell;
use std::default::Default;
use std::rc::{Rc, Weak};

use crate::collections::{Point, Vector};
use crate::objects::{Ray, Transform, Transformable};

#[derive(Debug, Default)]
pub struct Group {
    transform: Transform,
    parent: Option<Rc<RefCell<Group>>>,
    self_ref: Weak<RefCell<Group>>,
}

pub trait GroupTransformable {
    fn world_to_object_ray(&self, mut world_ray: Ray) -> Ray {
        if let Some(parent) = self.parent() {
            world_ray = parent.borrow().world_to_object_ray(world_ray);
        }

        world_ray.transform(&self.transformation_matrix().invert())
    }

    fn world_to_object_point(&self, mut world_point: Point) -> Point {
        if let Some(parent) = self.parent() {
            world_point = parent.borrow().world_to_object_point(world_point);
        }

        world_point.transform(&self.transformation_matrix().invert())
    }

    fn normal_to_world_vector(&self, object_normal: Vector) -> Vector {
        let object_normal = object_normal
            .transform(&self.transformation_matrix().invert().transpose())
            .normalise();

        if let Some(parent) = self.parent() {
            parent.borrow().normal_to_world_vector(object_normal)
        } else {
            object_normal
        }
    }

    fn transformation_matrix(&self) -> &Transform;
    fn transformation_matrix_mut(&mut self) -> &mut Transform;
    fn parent(&self) -> Option<Rc<RefCell<Group>>>;
    fn set_parent(&mut self, group: Rc<RefCell<Group>>);
}

impl Group {
    pub fn new<G: GroupTransformable>(
        transform: Transform,
        objects: Vec<&mut G>,
    ) -> Rc<RefCell<Group>> {
        let self_ref = Weak::new();
        let group = Rc::new(RefCell::new(Self {
            transform,
            parent: None,
            self_ref,
        }));
        group.borrow_mut().self_ref = Rc::downgrade(&Rc::clone(&group));

        for object in objects {
            object.set_parent(Rc::clone(&group))
        }

        group
    }

    pub fn add_object<G: GroupTransformable>(&mut self, object: &mut G)
    where
        G: GroupTransformable,
    {
        let group = self.self_ref.upgrade().unwrap();
        object.set_parent(group);
    }
}

impl GroupTransformable for Group {
    fn transformation_matrix(&self) -> &Transform {
        &self.transform
    }

    fn transformation_matrix_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn parent(&self) -> Option<Rc<RefCell<Group>>> {
        Option::clone(&self.parent)
    }

    fn set_parent(&mut self, group: Rc<RefCell<Group>>) {
        self.parent = Some(group);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::{Angle, Point, Vector};
    use crate::objects::{Axis, Ray, Shape, Sphere, TransformKind, Transformable};
    use crate::scenes::World;

    #[test]
    fn intersect_ray_with_group_in_world() {
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        *s2.transformation_matrix_mut() = Transform::new(TransformKind::Translate(0.0, 0.0, -3.0));
        let mut s3 = Sphere::default();
        *s3.transformation_matrix_mut() = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        let mut objects = vec![s1, s2, s3];

        let group = Group::new(Transform::default(), objects.iter_mut().collect());

        let objects = objects
            .into_iter()
            .map(|object: Sphere| -> Box<dyn Shape> { Box::new(object) })
            .collect();
        let world = World::new(objects, vec![]);
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let s1 = world.objects[0].as_ref();
        let s2 = world.objects[1].as_ref();

        let intersections = world.intersect_ray(&ray);
        assert_eq!(intersections.0.len(), 4);
        assert_eq!(intersections.0[0].object, s2);
        assert_eq!(intersections.0[1].object, s2);
        assert_eq!(intersections.0[2].object, s1);
        assert_eq!(intersections.0[3].object, s1);
    }

    use std::ops::DerefMut;

    #[test]
    fn intersect_transformed_group() {
        let mut s1 = Sphere::default();
        *s1.transformation_matrix_mut() = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        let mut objects = vec![s1];

        Group::new(
            Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)),
            objects.iter_mut().collect(),
        );

        let objects = objects
            .into_iter()
            .map(|object: Sphere| -> Box<dyn Shape> { Box::new(object) })
            .collect();
        let world = World::new(objects, vec![]);
        let ray = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = world.intersect_ray(&ray);
        println!("{:?}", world.objects[0].transformation_matrix());
        assert_eq!(intersections.0.len(), 2);
    }

    // #[test]
    // fn convert_point_from_world_to_object_space() {
    //     let mut s1 = Sphere::default();
    //     *s1.transformation_matrix_mut() = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
    //     let mut objects = vec![s1];

    //     let g2 = Group::new(
    //         Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)),
    //         objects.iter_mut().collect(),
    //     );
    //     let g1 = Group::new::<Group>(
    //         Transform::new(TransformKind::Rotate(
    //             Axis::Y,
    //             Angle::from_radians(std::f64::consts::FRAC_PI_2),
    //         )),
    //         vec![],
    //     );
    //     g1.borrow_mut().add_object(g2.borrow_mut().deref_mut());

    //     let world_point = Point::new(-2.0, 0.0, -10.0);
    //     let resulting_point = Point::new(0.0, 0.0, -1.0);
    //     assert_eq!(
    //         objects[0].world_to_object_point(world_point),
    //         resulting_point
    //     );
    // }

    // #[test]
    // fn convert_normal_from_object_to_world_space() {
    //     let mut s1 = Sphere::default();
    //     *s1.transformation_matrix_mut() = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
    //     let mut objects = vec![s1];

    //     let g2 = Group::new(
    //         Transform::new(TransformKind::Scale(1.0, 2.0, 3.0)),
    //         objects.iter_mut().collect(),
    //     );
    //     let g1 = Group::new::<Group>(
    //         Transform::new(TransformKind::Rotate(
    //             Axis::Y,
    //             Angle::from_radians(std::f64::consts::FRAC_PI_2),
    //         )),
    //         vec![],
    //     );
    //     g1.borrow_mut().add_object(g2.borrow_mut().deref_mut());

    //     let local_normal = Vector::new(
    //         3.0_f64.sqrt() / 3.0,
    //         3.0_f64.sqrt() / 3.0,
    //         3.0_f64.sqrt() / 3.0,
    //     );
    //     let resulting_normal = Vector::new(0.2857, 0.4286, -0.8571);
    //     assert_eq!(
    //         objects[0].normal_to_world_vector(local_normal),
    //         resulting_normal
    //     );
    // }
}
