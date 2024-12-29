use crate::collections::*;
use crate::objects::*;
use crate::utils::Preset;

#[derive(Default, Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<Light>>,
}

impl World {
    const MAX_RAYCAST_DEPTH: i32 = 10;

    pub fn new(objects: Vec<Box<dyn Shape>>, lights: Vec<Box<Light>>) -> World {
        World { objects, lights }
    }

    pub fn cast_ray(&self, ray: &Ray) -> Colour {
        self.shade_ray(ray, Self::MAX_RAYCAST_DEPTH)
    }

    fn shade_ray(&self, ray: &Ray, depth_remaining: i32) -> Colour {
        if depth_remaining == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let intersections = self.intersect_ray(ray);

        let computed_intersect = match intersections.hit() {
            Some(intersect) => intersect.precompute(),
            None => return Colour::new(0.0, 0.0, 0.0),
        };

        let surface = self.shade_surface(&computed_intersect);
        let reflected = self.shade_reflection(&computed_intersect, depth_remaining);
        let refracted = self.shade_refraction(&computed_intersect, &intersections, depth_remaining);

        let material = computed_intersect.object.material();
        if material.reflectance > 0.0 && material.transparency > 0.0 {
            let RefractionBoundary { n1, n2 } =
                Self::compute_refraction_boundary(&computed_intersect, &intersections);
            let reflectance = computed_intersect.schlick_reflectance(n1, n2);
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn intersect_ray<'a>(&'a self, ray: &'a Ray) -> Intersections<'a, dyn Shape> {
        self.objects
            .iter()
            .map(|object| object.intersect(ray))
            .fold(
                Intersections::default(),
                |mut intersections1, intersections2| {
                    intersections1.combine_intersections(intersections2);
                    intersections1
                },
            )
    }

    pub fn is_shadowed_point(&self, light: &Light, point: Point) -> bool {
        let vector = light.position - point;
        let distance = vector.magnitude();
        let direction = vector.normalise();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect_ray(&ray);

        matches!(intersections.hit(), Some(hit) if hit.t < distance)
    }

    fn shade_surface(&self, computed_intersect: &ComputedIntersect<dyn Shape>) -> Colour {
        let mut surface_colour = Colour::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            surface_colour = surface_colour
                + computed_intersect.shade(
                    light,
                    self.is_shadowed_point(light, computed_intersect.over_point),
                );
        }
        surface_colour
    }

    fn shade_reflection(
        &self,
        computed_intersect: &ComputedIntersect<dyn Shape>,
        depth_remaining: i32,
    ) -> Colour {
        if depth_remaining == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let reflected_ray = computed_intersect.reflected_ray;
        let reflectance = computed_intersect.object.material().reflectance;

        if reflectance == 0.0 {
            return Colour::new(0.0, 0.0, 0.0);
        };

        reflectance * self.shade_ray(&reflected_ray, depth_remaining - 1)
    }

    fn shade_refraction(
        &self,
        computed_intersect: &ComputedIntersect<dyn Shape>,
        intersections: &Intersections<dyn Shape>,
        depth_remaining: i32,
    ) -> Colour {
        if depth_remaining == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let transparency = computed_intersect.object.material().transparency;

        if transparency == 0.0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let RefractionBoundary { n1, n2 } =
            Self::compute_refraction_boundary(computed_intersect, intersections);

        let n_ratio = n1 / n2;
        let cos_i = computed_intersect.eyev.dot(computed_intersect.normal);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let refracted_direction = computed_intersect.normal * (n_ratio * cos_i - cos_t)
            - computed_intersect.eyev * n_ratio;
        let refracted_ray = Ray::new(computed_intersect.under_point, refracted_direction);

        transparency * self.shade_ray(&refracted_ray, depth_remaining - 1)
    }

    pub fn compute_refraction_boundary(
        computed_intersect: &ComputedIntersect<dyn Shape>,
        intersections: &Intersections<dyn Shape>,
    ) -> RefractionBoundary {
        let refraction_boundaries = intersections.compute_refraction_boundaries();
        let idx_intersect = intersections
            .0
            .iter()
            .position(|raw_intersect| raw_intersect.t == computed_intersect.t)
            .unwrap();
        refraction_boundaries[idx_intersect]
    }
}

impl Preset for World {
    fn preset() -> World {
        let s1 = Sphere::new(
            Transform::preset(),
            Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            },
        );
        let s2 = Sphere::new(
            Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            Material::preset(),
        );
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        World {
            objects: vec![Box::new(s1), Box::new(s2)],
            lights: vec![Box::new(light)],
        }
    }
}

impl<S: 'static + Shape> From<S> for World {
    fn from(value: S) -> Self {
        World::new(vec![Box::new(value)], vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn cast_ray() {
    //     let world = World::preset();
    //     let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let resulting_colour = Colour::new(0.38066, 0.47583, 0.28550);
    //     assert_eq!(world.cast_ray(&ray), resulting_colour);
    // }

    // #[test]
    // fn cast_ray_inside() {
    //     let world = World {
    //         lights: vec![Light::new(
    //             Point::new(0.0, 0.25, 0.0),
    //             Colour::new(1.0, 1.0, 1.0),
    //         )],
    //         ..World::preset()
    //     };
    //     let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    //     let resulting_colour = Colour::new(0.90498, 0.90498, 0.90498);
    //     assert_eq!(world.cast_ray(&ray), resulting_colour);
    // }

    #[test]
    fn cast_ray_misses() {
        let world = World::preset();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(world.cast_ray(&ray), resulting_colour);
    }

    // #[test]
    // fn cast_ray_hits() {
    //     let world = World::preset();
    //     let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let resulting_colour = Colour::new(0.38066, 0.47583, 0.28550);
    //     assert_eq!(world.cast_ray(&ray), resulting_colour);
    // }

    #[test]
    fn cast_ray_intersects_behind() {
        let s1 = Sphere::new(
            Transform::preset(),
            Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                ambient: 1.0,
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            },
        );
        let s2 = Sphere::new(
            Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            Material {
                ambient: 1.0,
                ..Material::preset()
            },
        );
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World::new(vec![Box::new(s1), Box::new(s2)], vec![Box::new(light)]);
        let inner = &world.objects[1];
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let resulting_colour = inner
            .material()
            .pattern
            .colour_at(Point::new(0.0, 0.0, 0.0));
        assert_eq!(world.cast_ray(&ray), resulting_colour);
    }

    #[test]
    fn no_shadow_nothing_collinear() {
        let world = World::preset();
        let point = Point::new(0.0, 10.0, 0.0);
        assert!(!world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn shadow_collinear() {
        let world = World::preset();
        let point = Point::new(10.0, -10.0, 10.0);
        assert!(world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn no_shadow_object_behind_light() {
        let world = World::preset();
        let point = Point::new(-20.0, 20.0, -20.0);
        assert!(!world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn no_shadow_object_behind_point() {
        let world = World::preset();
        let point = Point::new(-2.0, 2.0, -2.0);
        assert!(!world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn cast_ray_hit_in_shadow() {
        let s1 = Sphere::preset();
        let s2 = Sphere::new(
            Transform::new(TransformKind::Translate(0.0, 0.0, 10.0)),
            Material::preset(),
        );
        let s2_clone = Sphere::new(
            Transform::new(TransformKind::Translate(0.0, 0.0, 10.0)),
            Material::preset(),
        );
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World::new(
            vec![Box::new(s1), Box::new(s2_clone)],
            vec![Box::new(light)],
        );
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let raw_intersect = RawIntersect::new(4.0, &s2, &ray, None);
        let computed_intersect = raw_intersect.precompute();
        let resulting_colour = Colour::new(0.1, 0.1, 0.1);
        assert_eq!(
            computed_intersect.shade(
                &world.lights[0],
                world.is_shadowed_point(&world.lights[0], computed_intersect.target),
            ),
            resulting_colour
        );
    }

    #[test]
    fn reflected_colour_for_nonreflective_material() {
        let mut world = World::preset();
        let ray = Ray::new(Point::new(-2.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let shape = world.objects[1].as_mut();
        shape.material_mut().ambient = -1.0;

        let shape = world.objects[1].as_ref();
        let raw_intersect = RawIntersect::new(-1.0, shape, &ray, None);
        let computed_intersect = raw_intersect.precompute();
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(
            world.shade_reflection(&computed_intersect, 10),
            resulting_colour
        );
    }

    // #[test]
    // fn reflected_colour_for_reflective_material() {
    //     let world = World::preset();
    //     let ray = Ray::new(
    //         Point::new(0.0, 0.0, -3.0),
    //         Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    //     );
    //     let mut shape = Plane::preset();
    //     shape.material_mut().reflectance = 0.5;
    //     *shape.transformation_matrix_mut() =
    //         Transform::new(TransformKind::Translate(0.0, -1.0, 0.0));
    //     let raw_intersect = RawIntersect::new(2.0_f64.sqrt(), &shape as &dyn Shape, &ray);
    //     let computed_intersect = raw_intersect.precompute();
    //     let resulting_colour = Colour::new(0.19032, 0.23790, 0.14274);
    //     assert_eq!(
    //         world.shade_reflection(&computed_intersect, 10),
    //         resulting_colour
    //     );
    // }

    // #[test]
    // fn shade_hit_reflective_material() {
    //     let mut world = World::preset();
    //     let mut shape = Plane::preset();
    //     shape.material_mut().reflectance = 0.5;
    //     *shape.transformation_matrix_mut() =
    //         Transform::new(TransformKind::Translate(0.0, -1.0, 0.0));
    //     world.objects.push(Box::new(shape));
    //
    //     let ray = Ray::new(
    //         Point::new(0.0, 0.0, -3.0),
    //         Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    //     );
    //     let resulting_colour = Colour::new(0.87677, 0.92436, 0.82918);
    //     assert_eq!(world.cast_ray(&ray), resulting_colour)
    // }

    #[test]
    fn shade_hit_mutually_reflective_surfaces() {
        let world = World {
            objects: vec![
                Box::new(Plane::new(
                    Transform::new(TransformKind::Translate(0.0, -1.0, 0.0)),
                    Material {
                        reflectance: 1.0,
                        ..Material::preset()
                    },
                )),
                Box::new(Plane::new(
                    Transform::new(TransformKind::Translate(0.0, 1.0, 0.0)),
                    Material {
                        reflectance: 1.0,
                        ..Material::preset()
                    },
                )),
            ],
            lights: vec![Box::new(Light::new(
                Point::new(0.0, 0.0, 0.0),
                Colour::new(1.0, 1.0, 1.0),
            ))],
        };

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        world.cast_ray(&ray);
    }

    #[test]
    fn refractive_indices_at_various_intersections() {
        let sphere1 = Sphere::new(
            Transform::new(TransformKind::Scale(2.0, 2.0, 2.0)),
            Material {
                refractive_index: 1.5,
                ..Sphere::glass_sphere().material
            },
        );
        let sphere2 = Sphere::new(
            Transform::new(TransformKind::Translate(0.0, 0.0, -0.25)),
            Material {
                refractive_index: 2.0,
                ..Sphere::glass_sphere().material
            },
        );
        let sphere3 = Sphere::new(
            Transform::new(TransformKind::Translate(0.0, 0.0, 0.25)),
            Material {
                refractive_index: 2.5,
                ..Sphere::glass_sphere().material
            },
        );
        let world = World::new(
            vec![Box::new(sphere1), Box::new(sphere2), Box::new(sphere3)],
            vec![],
        );
        let ray = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = world.intersect_ray(&ray);

        let test_cases: [(usize, f64, f64); 6] = [
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];
        for (idx_intersect, n1, n2) in test_cases {
            let refractive_indices = intersections.compute_refraction_boundaries();
            assert_eq!(refractive_indices[idx_intersect].n1, n1);
            assert_eq!(refractive_indices[idx_intersect].n2, n2);
        }
    }

    #[test]
    fn refracted_colour_of_opaque_object() {
        let world = World::preset();
        let shape = world.objects[0].as_ref();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        let intersections = Intersections::new(vec![
            RawIntersect::new(4.0, shape, &ray, None),
            RawIntersect::new(6.0, shape, &ray, None),
        ]);
        let computed_intersect = intersections.hit().unwrap().precompute();
        assert_eq!(
            world.shade_refraction(&computed_intersect, &intersections, 10),
            resulting_colour
        );
    }

    #[test]
    fn refracted_colour_under_total_internal_reflection() {
        let mut world = World::preset();
        let shape = world.objects[0].as_mut();
        let material = shape.material_mut();
        material.transparency = 1.0;
        material.refractive_index = 1.5;

        let shape = world.objects[0].as_ref();
        let ray = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let intersections = Intersections::new(vec![
            RawIntersect::new(-2.0_f64.sqrt() / 2.0, shape, &ray, None),
            RawIntersect::new(2.0_f64.sqrt() / 2.0, shape, &ray, None),
        ]);
        let computed_intersect = intersections[1].precompute();
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(
            world.shade_refraction(&computed_intersect, &intersections, 10),
            resulting_colour
        );
    }

    // #[test]
    // fn refracted_colour_from_refracted_ray() {
    //     let mut world = World::preset();
    //     let shape1 = world.objects[0].as_mut();
    //     let material = shape1.material_mut();
    //     material.ambient = 1.0;
    //     material.pattern = Box::new(crate::utils::presets::TestPattern {
    //         transform: Transform::new(TransformKind::Identity),
    //     });
    //
    //     let shape2 = world.objects[1].as_mut();
    //     let material = shape2.material_mut();
    //     material.transparency = 1.0;
    //     material.refractive_index = 1.5;
    //
    //     let shape1 = world.objects[0].as_ref();
    //     let shape2 = world.objects[1].as_ref();
    //
    //     let ray = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
    //     let intersections = Intersections::new(vec![
    //         RawIntersect::new(-0.98990, shape1, &ray),
    //         RawIntersect::new(-0.48990, shape2, &ray),
    //         RawIntersect::new(0.48990, shape2, &ray),
    //         RawIntersect::new(0.98990, shape1, &ray),
    //     ]);
    //     let computed_intersect = intersections.0[2].precompute();
    //     let resulting_colour = Colour::new(0.0, 0.99888, 0.04725);
    //     assert_eq!(
    //         world.shade_refraction(&computed_intersect, &intersections, 10),
    //         resulting_colour
    //     );
    // }

    // #[test]
    // fn refracted_colour() {
    //     let mut world = World::preset();
    //     let mut floor = Plane::preset();
    //     *floor.transformation_matrix_mut() =
    //         Transform::new(TransformKind::Translate(0.0, -1.0, 0.0));
    //     let floor_material = floor.material_mut();
    //     floor_material.reflectance = 0.5;
    //     floor_material.transparency = 0.5;
    //     floor_material.refractive_index = 1.5;
    //     world.objects.push(Box::new(floor));
    //
    //     let ball = Sphere {
    //         transform: Transform::new(TransformKind::Translate(0.0, -3.5, -0.5)),
    //         material: Material {
    //             pattern: Box::new(Solid::new(Colour::new(1.0, 0.0, 0.0))),
    //             ambient: 0.5,
    //             ..Material::preset()
    //         },
    //     };
    //     world.objects.push(Box::new(ball));
    //
    //     let ray = Ray::new(
    //         Point::new(0.0, 0.0, -3.0),
    //         Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    //     );
    //
    //     let resulting_colour = Colour::new(0.93391, 0.69643, 0.69243);
    //     assert_eq!(world.cast_ray(&ray), resulting_colour);
    // }
}
