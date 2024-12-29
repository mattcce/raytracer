use crate::objects::*;

#[derive(Debug)]
pub struct Csg {
    csg_operation: CsgOperation,
    lshape: Box<Shape>,
    rshape: Box<Shape>,
    bounds: Bounds,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CsgOperation {
    Union,
    Intersect,
    Difference,
}

impl Csg {
    pub fn new(csg_operation: CsgOperation, lshape: Shape, rshape: Shape) -> Csg {
        let bounds =
            Bounds::Checked(lshape.bounds().bounding_box() + rshape.bounds().bounding_box());

        Csg {
            csg_operation,
            lshape: Box::new(lshape),
            rshape: Box::new(rshape),
            bounds,
        }
    }

    pub fn csg_operation(&self) -> CsgOperation {
        self.csg_operation
    }

    pub fn lshape(&self) -> &Shape {
        self.lshape.as_ref()
    }

    pub fn rshape(&self) -> &Shape {
        self.rshape.as_ref()
    }

    fn evaluate_intersections<'a>(
        &self,
        hit_register: HitRegister<'a, dyn PrimitiveShape>,
    ) -> HitRegister<'a, dyn PrimitiveShape> {
        let hits = hit_register.expose();

        let mut in_left = false;
        let mut in_right = false;

        let mut hit_register = HitRegister::empty();

        let intersection_evaluator = match self.csg_operation {
            CsgOperation::Union => Csg::union_evaluate_intersection,
            CsgOperation::Intersect => Csg::intersect_evaluate_intersection,
            CsgOperation::Difference => Csg::difference_evaluate_intersection,
        };

        for hit in hits {
            let lhit = self.lshape().contains(hit.object());

            if intersection_evaluator(lhit, in_left, in_right) {
                hit_register.add_raw_intersect(hit);
            }

            if lhit {
                in_left = !in_left;
            } else {
                in_right = !in_right;
            }
        }

        hit_register
    }

    fn union_evaluate_intersection(left_hit: bool, in_left: bool, in_right: bool) -> bool {
        (left_hit && !in_right) || (!left_hit && !in_left)
    }

    fn intersect_evaluate_intersection(left_hit: bool, in_left: bool, in_right: bool) -> bool {
        (left_hit && in_right) || (!left_hit && in_left)
    }

    fn difference_evaluate_intersection(left_hit: bool, in_left: bool, in_right: bool) -> bool {
        (left_hit && !in_right) || (!left_hit && in_left)
    }
}

impl Intersectable<dyn PrimitiveShape> for Csg {
    fn intersect_ray<'world: 'ray, 'ray>(
        &'world self,
        world_ray: &'ray Ray,
        transform_stack: Vec<&'ray Transform>,
    ) -> HitRegister<'ray, dyn PrimitiveShape> {
        let mut lshape_hit_register = self
            .lshape()
            .intersect_ray(world_ray, transform_stack.clone());
        let rshape_hit_register = self
            .rshape()
            .intersect_ray(world_ray, transform_stack.clone());
        lshape_hit_register.combine_registers(rshape_hit_register);

        self.evaluate_intersections(lshape_hit_register)
    }
}

impl Bounded for Csg {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::{Point, Vector};
    use crate::utils::{BuildInto, Buildable};

    #[test]
    fn evaluate_intersections_for_union() {
        let test_cases = [
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, false),
            (true, false, false, true),
            (false, true, true, false),
            (false, true, false, false),
            (false, false, true, true),
            (false, false, false, true),
        ];
        for (lhit, inl, inr, result) in test_cases {
            assert_eq!(Csg::union_evaluate_intersection(lhit, inl, inr), result);
        }
    }

    #[test]
    fn evaluate_intersections_for_intersect() {
        let test_cases = [
            (true, true, true, true),
            (true, true, false, false),
            (true, false, true, true),
            (true, false, false, false),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, false),
            (false, false, false, false),
        ];
        for (lhit, inl, inr, result) in test_cases {
            assert_eq!(Csg::intersect_evaluate_intersection(lhit, inl, inr), result);
        }
    }

    #[test]
    fn evaluate_intersections_for_difference() {
        let test_cases = [
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, false),
            (true, false, false, true),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, false),
            (false, false, false, false),
        ];
        for (lhit, inl, inr, result) in test_cases {
            assert_eq!(
                Csg::difference_evaluate_intersection(lhit, inl, inr),
                result
            );
        }
    }

    #[test]
    fn filter_list_of_intersections() {
        let csg_union = Csg::new(
            CsgOperation::Union,
            Sphere::builder().build_into(),
            Cube::builder().build_into(),
        );
        let csg_intersect = Csg::new(
            CsgOperation::Intersect,
            Sphere::builder().build_into(),
            Cube::builder().build_into(),
        );
        let csg_difference = Csg::new(
            CsgOperation::Difference,
            Sphere::builder().build_into(),
            Cube::builder().build_into(),
        );
        let test_cases = [
            (csg_union, 0.0, 3.0),
            (csg_intersect, 1.0, 2.0),
            (csg_difference, 0.0, 1.0),
        ];
        let placeholder_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0));

        for (csg, x0, x1) in test_cases {
            let Shape::Primitive(lshape) = csg.lshape() else {
                panic!();
            };
            let lshape = lshape.as_ref();
            let Shape::Primitive(rshape) = csg.rshape() else {
                panic!();
            };
            let rshape = rshape.as_ref();

            let hit_register = HitRegister::from(vec![
                Intersect::new(0.0, lshape, &placeholder_ray, None, vec![]),
                Intersect::new(1.0, rshape, &placeholder_ray, None, vec![]),
                Intersect::new(2.0, lshape, &placeholder_ray, None, vec![]),
                Intersect::new(3.0, rshape, &placeholder_ray, None, vec![]),
            ]);

            let filtered_intersections = csg.evaluate_intersections(hit_register).expose();
            let t_list: Vec<f64> = filtered_intersections.iter().map(|itx| itx.t()).collect();
            assert_eq!(x0, t_list[0]);
            assert_eq!(x1, t_list[1]);
        }
    }

    #[test]
    fn no_intersection_with_csg() {
        let c = Csg::new(
            CsgOperation::Union,
            Sphere::builder().build_into(),
            Cube::builder().build_into(),
        );
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert!(c.intersect_ray(&ray, vec![]).finalise_hit().is_none());
    }

    #[test]
    fn intersections_with_csg() {
        let s1: Shape = Sphere::builder().build_into();
        let s2: Shape = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, 0.5)))
            .build_into();
        let c = Csg::new(CsgOperation::Union, s1, s2);
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = c.intersect_ray(&ray, vec![]).expose();
        assert_eq!(intersections[0].t(), 4.0);
        assert!(c.lshape().contains(intersections[0].object()));
        assert_eq!(intersections[1].t(), 6.5);
        assert!(c.rshape().contains(intersections[1].object()));
    }
}
