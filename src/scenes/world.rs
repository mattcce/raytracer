use crate::collections::*;
use crate::objects::*;
use crate::utils::{Preset, Shape};

#[derive(Default, Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<Light>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Shape>>, lights: Vec<Box<Light>>) -> World {
        World { objects, lights }
    }

    pub fn cast_ray(&self, ray: &Ray) -> Colour {
        let intersections = self.intersect_ray(ray);

        let computed_intersect = match intersections.hit() {
            Some(intersect) => intersect.precompute(),
            None => return Colour::new(0.0, 0.0, 0.0),
        };
        let mut resulting_colour = Colour::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            resulting_colour = resulting_colour
                + computed_intersect.shade(
                light,
                self.is_shadowed_point(light, computed_intersect.over_point),
            );
        }
        resulting_colour
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
}

impl Preset for World {
    fn preset() -> World {
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            },
            ..Sphere::preset()
        };
        let s2 = Sphere {
            transform: Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            ..Sphere::preset()
        };
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        World {
            objects: vec![Box::new(s1), Box::new(s2)],
            lights: vec![Box::new(light)],
        }
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
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                ambient: 1.0,
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            },
            ..Sphere::preset()
        };
        let s2 = Sphere {
            material: Material {
                ambient: 1.0,
                ..Material::preset()
            },
            transform: Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
        };
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World::new(vec![Box::new(s1), Box::new(s2)], vec![Box::new(light)]);
        let inner = &world.objects[1];
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let resulting_colour = inner.material().colour;
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
        let s2 = Sphere {
            transform: Transform::new(TransformKind::Translate(0.0, 0.0, 10.0)),
            ..Sphere::preset()
        };
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World::new(
            vec![Box::new(s1), Box::new(s2.clone())],
            vec![Box::new(light)],
        );
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let raw_intersect = RawIntersect::new(4.0, &s2, &ray);
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
}
