use crate::collections::*;
use crate::objects::*;

#[derive(Default, Debug)]
pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<Light>,
}

impl<'world: 'ray, 'ray> World {
    const MAX_RAYCAST_DEPTH: i32 = 10;

    pub fn new(objects: Vec<Shape>, lights: Vec<Light>) -> World {
        World { objects, lights }
    }

    pub fn cast_ray(&self, ray: Ray) -> Colour {
        self.shade_ray(&ray, Self::MAX_RAYCAST_DEPTH)
    }

    fn shade_ray(&self, ray: &Ray, depth_remaining: i32) -> Colour {
        if depth_remaining == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let hit_register = self.intersect_ray(ray);

        if let Some(computed_intersect) = hit_register.finalise_hit() {
            let surface = self.shade_surface(&computed_intersect);
            let reflected = self.shade_reflection(&computed_intersect, depth_remaining);
            let refracted = self.shade_refraction(&computed_intersect, depth_remaining);

            let material = computed_intersect.object().material();
            if material.reflectance > 0.0 && material.transparency > 0.0 {
                let reflectance = computed_intersect.schlick_reflectance();
                surface + reflected * reflectance + refracted * (1.0 - reflectance)
            } else {
                surface + reflected + refracted
            }
        } else {
            return Colour::new(0.0, 0.0, 0.0);
        }
    }

    pub(crate) fn intersect_ray(
        &'world self,
        ray: &'ray Ray,
    ) -> HitRegister<'ray, dyn PrimitiveShape> {
        let mut ray_hit_register = HitRegister::empty();

        for shape in &self.objects {
            match shape {
                Shape::Primitive(primitive_shape) => {
                    let shape_hit_register = primitive_shape.intersect_ray(ray, vec![]);
                    ray_hit_register.combine_registers(shape_hit_register);
                }
                Shape::Group(group) => {
                    let shape_hit_register = group.intersect_ray(ray, vec![]);
                    ray_hit_register.combine_registers(shape_hit_register);
                }
            }
        }

        ray_hit_register
    }

    fn is_shadowed_point(&self, light: &Light, point: Point) -> bool {
        let vector = light.position - point;
        let distance = vector.magnitude();
        let direction = vector.normalise();

        let ray = Ray::new(point, direction);
        let hit_register = self.intersect_ray(&ray);

        matches!(hit_register.finalise_hit(), Some(hit) if hit.t() < distance)
    }

    fn shade_surface(
        &self,
        computed_intersect: &Intersect<dyn PrimitiveShape, Computed>,
    ) -> Colour {
        let mut surface_colour = Colour::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            surface_colour = surface_colour
                + computed_intersect.shade(
                    light,
                    self.is_shadowed_point(light, computed_intersect.over_point()),
                );
        }
        surface_colour
    }

    fn shade_reflection(
        &self,
        computed_intersect: &Intersect<dyn PrimitiveShape, Computed>,
        depth_remaining: i32,
    ) -> Colour {
        if depth_remaining == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let reflected_ray = computed_intersect.reflected_ray();
        let reflectance = computed_intersect.object().material().reflectance;

        if reflectance == 0.0 {
            return Colour::new(0.0, 0.0, 0.0);
        };

        reflectance * self.shade_ray(&reflected_ray, depth_remaining - 1)
    }

    fn shade_refraction(
        &self,
        computed_intersect: &Intersect<dyn PrimitiveShape, Computed>,
        depth_remaining: i32,
    ) -> Colour {
        if depth_remaining == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let transparency = computed_intersect.object().material().transparency;

        if transparency == 0.0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let (n1, n2) = computed_intersect.refraction_boundary();

        let n_ratio = n1 / n2;
        let cos_i = computed_intersect.eyev().dot(computed_intersect.normal());
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let refracted_direction = computed_intersect.normal() * (n_ratio * cos_i - cos_t)
            - computed_intersect.eyev() * n_ratio;
        let refracted_ray = Ray::new(computed_intersect.under_point(), refracted_direction);

        transparency * self.shade_ray(&refracted_ray, depth_remaining - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::approx_eq;

    #[test]
    fn cast_ray() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let colour = world.cast_ray(ray);
        let resulting_colour = Colour::new(0.380661, 0.475826, 0.285496);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn cast_ray_inside() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(0.0, 0.25, 0.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let colour = world.cast_ray(ray);
        let resulting_colour = Colour::new(0.904984, 0.904984, 0.904984);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn cast_ray_misses() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(world.cast_ray(ray), resulting_colour);
    }

    #[test]
    fn cast_ray_hits() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let colour = world.cast_ray(ray);
        let resulting_colour = Colour::new(0.380661, 0.475826, 0.285496);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn cast_ray_intersects_behind() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                ambient: 1.0,
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material {
                ambient: 1.0,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World::new(vec![s1, s2], vec![light]);
        let inner = &world.objects[1];
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        if let Shape::Primitive(shape) = inner {
            let resulting_colour = shape
                .material()
                .pattern
                .colour_at(Point::new(0.0, 0.0, 0.0));
            assert_eq!(world.cast_ray(ray), resulting_colour);
        } else {
            panic!();
        }
    }

    #[test]
    fn no_shadow() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        assert!(!world.is_shadowed_point(&world.lights[0], Point::new(0.0, 10.0, 0.0)));
    }

    #[test]
    fn no_shadow_nothing_collinear() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let point = Point::new(0.0, 10.0, 0.0);
        assert!(!world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn shadow_collinear() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let point = Point::new(10.0, -10.0, 10.0);
        assert!(world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn no_shadow_object_behind_light() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let point = Point::new(-20.0, 20.0, -20.0);
        assert!(!world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn no_shadow_object_behind_point() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let point = Point::new(-2.0, 2.0, -2.0);
        assert!(!world.is_shadowed_point(&world.lights[0], point));
    }

    #[test]
    fn cast_ray_hit_in_shadow() {
        let s1 = Sphere::builder().set_material(Material::preset()).wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 0.0, 10.0)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World::new(vec![s1, s2], vec![light]);
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let computed_intersect = world.intersect_ray(&ray).finalise_hit().unwrap();
        let resulting_colour = Colour::new(0.1, 0.1, 0.1);
        assert_eq!(
            computed_intersect.shade(
                &world.lights[0],
                world.is_shadowed_point(&world.lights[0], computed_intersect.target()),
            ),
            resulting_colour
        );
    }

    #[test]
    fn reflected_colour_for_nonreflective_material() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material {
                ambient: 1.0,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let computed_intersect = world.intersect_ray(&ray).finalise_hit().unwrap();
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(
            world.shade_reflection(&computed_intersect, 10),
            resulting_colour
        );
    }

    #[test]
    fn reflected_colour_for_reflective_material() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let s3 = Plane::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, -1.0, 0.0)))
            .set_material(Material {
                reflectance: 0.5,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2, s3],
            lights: vec![light],
        };
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let computed_intersect = world.intersect_ray(&ray).finalise_hit().unwrap();
        let colour = world.shade_reflection(&computed_intersect, 10);
        let resulting_colour = Colour::new(0.190331, 0.237913, 0.142748);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn shade_hit_reflective_material() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let s3 = Plane::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, -1.0, 0.0)))
            .set_material(Material {
                reflectance: 0.5,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2, s3],
            lights: vec![light],
        };
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let colour = world.cast_ray(ray);
        let resulting_colour = Colour::new(0.876756, 0.924339, 0.829173);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn shade_hit_mutually_reflective_surfaces() {
        let s1 = Plane::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, -1.0, 0.0)))
            .set_material(Material {
                reflectance: 1.0,
                ..Material::preset()
            })
            .wrap();
        let s2 = Plane::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, 1.0, 0.0)))
            .set_material(Material {
                reflectance: 1.0,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(0.0, 0.0, 0.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        // the following method call should terminate in finite time
        world.cast_ray(ray);
    }

    #[test]
    fn refracted_colour_of_opaque_object() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let computed_intersect = world.intersect_ray(&ray).finalise_hit().unwrap();
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(
            world.shade_refraction(&computed_intersect, 10),
            resulting_colour
        );
    }

    #[test]
    fn refracted_colour_under_total_internal_reflection() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                transparency: 1.0,
                refractive_index: 1.5,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let computed_intersect = world.intersect_ray(&ray).finalise_hit().unwrap();
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(
            world.shade_refraction(&computed_intersect, 10),
            resulting_colour
        );
    }

    #[derive(Debug)]
    struct TestPattern {
        frame_transformation: Transform,
    }

    impl TestPattern {
        fn new(frame_transformation: Transform) -> TestPattern {
            TestPattern {
                frame_transformation,
            }
        }
    }

    impl Pattern for TestPattern {
        fn frame_transformation(&self) -> &Transform {
            &self.frame_transformation
        }

        fn local_colour_at(&self, pattern_point: Point) -> Colour {
            let Point { x, y, z } = pattern_point;
            Colour::new(x, y, z)
        }
    }

    #[test]
    fn refracted_colour_from_refracted_ray() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(TestPattern::new(Transform::default())),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1.0,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        let ray = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let computed_intersect = world.intersect_ray(&ray).finalise_hit().unwrap();
        let colour = world.shade_refraction(&computed_intersect, 10);
        let resulting_colour = Colour::new(0.0, 0.998884, 0.047216);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn refracted_colour() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .wrap();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .wrap();
        let s3 = Plane::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, -1.0, 0.0)))
            .set_material(Material {
                reflectance: 0.5,
                transparency: 0.5,
                refractive_index: 1.5,
                ..Material::preset()
            })
            .wrap();
        let s4 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Translate(0.0, -3.5, -0.5)))
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(1.0, 0.0, 0.0))),
                ambient: 0.5,
                ..Material::preset()
            })
            .wrap();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2, s3, s4],
            lights: vec![light],
        };

        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let colour = world.cast_ray(ray);
        let resulting_colour = Colour::new(0.933915, 0.696434, 0.692431);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn intersection_retrieves_interpolated_normal() {
        let smooth_triangle = SmoothTriangle::builder()
            .set_vertices([
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ])
            .set_normals([
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(-1.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
            ])
            .wrap();
        let ray = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
        let world = World::new(vec![smooth_triangle], vec![]);
        let normal = world.intersect_ray(&ray).finalise_hit().unwrap().normal();
        let resulting_normal = Vector::new(-0.5547, 0.83205, 0.0);
        approx_eq!(normal.x, resulting_normal.x);
        approx_eq!(normal.y, resulting_normal.y);
        approx_eq!(normal.z, resulting_normal.z);
    }
}
