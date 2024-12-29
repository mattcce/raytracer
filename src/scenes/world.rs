use crate::collections::*;
use crate::objects::*;
use crate::utils::Preset;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, lights: Vec<Light>) -> World {
        World { objects, lights }
    }

    pub fn cast_ray(&self, ray: &Ray) -> Colour {
        let intersections = self.intersect(&ray);
        let mut resulting_colour = Colour::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            resulting_colour = resulting_colour + intersections.shade_hit(*light);
        }
        resulting_colour
    }
}

impl Default for World {
    fn default() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
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
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}

impl Intersectable for World {
    fn intersect<'a>(&'a self, ray: &'a Ray) -> Intersections<'a> {
        let mut intersections = Intersections(vec![]);
        for object in &self.objects {
            let object_intersections = object.intersect(ray);
            intersections.combine_intersections(object_intersections);
        }
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_ray_with_world() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let world = World::preset();
        let intersections = world.intersect(&ray);
        assert_eq!(intersections.0.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }

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
        let mut world = World::preset();
        let mut outer = &mut world.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = &mut world.objects[1];
        inner.material.ambient = 1.0;
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let resulting_colour = inner.material.colour;
        assert_eq!(world.cast_ray(&ray), resulting_colour);
    }
}
