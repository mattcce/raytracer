use super::*;
use crate::collections::{Colour, Point, Vector};
use crate::objects::PointLight;
use crate::scenes::World;

// this entire module needs to be refactored properly - there is really
// not much reason to perform pre-computations and box them up even into
// different functions that serve no meaningful purpose by themselves
// aside to only be immediately pumped into another function.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comps<'a> {
    t: f64,
    object: &'a Sphere,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool,
}

impl<'a> Comps<'a> {
    fn prepare(intersect: &'a Intersect<'a>, ray: Ray) -> Comps<'a> {
        let t = intersect.t();
        let object = intersect.object();
        let point = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = object.normal_at(point);
        let inside = match normalv.dot(eyev) {
            _x if _x < 0.0 => {
                normalv = -normalv;
                true
            }
            _x if _x >= 0.0 => false,
            _ => panic!(),
        };
        Comps {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }

    fn lighting(
        material: Material,
        light: PointLight,
        illuminated_point: Point,
        eyev: Vector,
        normal: Vector,
    ) -> Colour {
        let effective_colour = material.colour * light.intensity;
        let lightv = (light.position - illuminated_point).normalise();
        let ambient = effective_colour * material.ambient;
        let light_dot_normal = lightv.dot(normal);
        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = Colour::new(0.0, 0.0, 0.0);
            specular = Colour::new(0.0, 0.0, 0.0);
        } else {
            diffuse = effective_colour * material.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normal);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Colour::new(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }

    fn shade_hit(&self, light: PointLight) -> Colour {
        Comps::lighting(
            self.object.material,
            light,
            self.point,
            self.eyev,
            self.normalv,
        )
    }

    pub fn colour_at(world: World, ray: Ray) -> Colour {
        let intersect = ray.intersect(&world);
        match intersect.hit() {
            Some(hit) => Comps::prepare(&hit, ray).shade_hit(world.lights[0]),
            None => Colour::new(0.0, 0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Intersect, Ray};

    #[test]
    fn prepare_comps_ray_outside() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let intersect = Intersect::new(4.0, &shape);
        let resulting_comps = Comps {
            t: intersect.t(),
            object: intersect.object(),
            point: Point::new(0.0, 0.0, -1.0),
            eyev: Vector::new(0.0, 0.0, -1.0),
            normalv: Vector::new(0.0, 0.0, -1.0),
            inside: false,
        };
        assert_eq!(Comps::prepare(&intersect, ray), resulting_comps);
    }

    #[test]
    fn prepare_comps_ray_inside() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let intersect = Intersect::new(1.0, &shape);
        let prepared_comps = Comps::prepare(&intersect, ray);
        assert_eq!(prepared_comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(prepared_comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(prepared_comps.inside, true);
        assert_eq!(prepared_comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn eye_directly_between_light_and_surface() {
        let material = Material::default();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let pointlight = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(1.9, 1.9, 1.9);
        assert_eq!(
            Comps::lighting(material, pointlight, position, eyev, normalv),
            resulting_colour
        );
    }

    #[test]
    fn eye_between_light_and_surface_eye_offset_45_degrees() {
        let material = Material::default();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let pointlight = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(1.0, 1.0, 1.0);
        assert_eq!(
            Comps::lighting(material, pointlight, position, eyev, normalv),
            resulting_colour
        );
    }

    //     #[test]
    //     fn eye_between_light_and_surface_light_offset_45_degrees() {
    //         let material = Material::default();
    //         let position = Point::zero();
    //         let eyev = Vector::new(0.0, 0.0, -1.0);
    //         let normalv = Vector::new(0.0, 0.0, -1.0);
    //         let pointlight = PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
    //         let resulting_colour = Colour::new(0.7364, 0.7364, 0.7364);
    //         assert_eq!(
    //             lighting(material, pointlight, position, eyev, normalv),
    //             resulting_colour
    //         );
    //     }

    //     #[test]
    //     fn eye_in_path_of_reflection_vector() {
    //         let material = Material::default();
    //         let position = Point::zero();
    //         let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    //         let normalv = Vector::new(0.0, 0.0, -1.0);
    //         let pointlight = PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
    //         let resulting_colour = Colour::new(1.6364, 1.6364, 1.6364);
    //         assert_eq!(
    //             lighting(material, pointlight, position, eyev, normalv),
    //             resulting_colour
    //         );
    //     }

    #[test]
    fn light_behind_surface() {
        let material = Material::default();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let pointlight = PointLight::new(Point::new(0.0, 0.0, 10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(0.1, 0.1, 0.1);
        assert_eq!(
            Comps::lighting(material, pointlight, position, eyev, normalv),
            resulting_colour
        );
    }

    // #[test]
    // fn shade_hit() {
    //     let world = World::default();
    //     let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let shape = &world.objects[0];
    //     let intersect = Intersect::new(4.0, shape);
    //     let comps = Comps::prepare(&intersect, ray);
    //     let resulting_colour = Colour::new(0.38066, 0.47583, 0.28550);
    //     assert_eq!(comps.shade_hit(world.lights[0]), resulting_colour);
    // }

    // #[test]
    // fn shade_hit_inside() {
    //     let world = World {
    //         lights: vec![PointLight::new(
    //             Point::new(0.0, 0.25, 0.0),
    //             Colour::new(1.0, 1.0, 1.0),
    //         )],
    //         ..World::default()
    //     };
    //     let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    //     let shape = &world.objects[1];
    //     let intersect = Intersect::new(0.5, shape);
    //     let comps = Comps::prepare(&intersect, ray);
    //     let resulting_colour = Colour::new(0.90498, 0.90498, 0.90498);
    //     assert_eq!(comps.shade_hit(world.lights[0]), resulting_colour);
    // }

    #[test]
    fn ray_misses() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let resulting_colour = Colour::new(0.0, 0.0, 0.0);
        assert_eq!(Comps::colour_at(world, ray), resulting_colour);
    }

    // #[test]
    // fn ray_hits() {
    //     let world = World::default();
    //     let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let resulting_colour = Colour::new(0.38066, 0.47583, 0.28550);
    //     assert_eq!(Comps::colour_at(world, ray), resulting_colour);
    // }

    #[test]
    fn ray_intersects_behind() {
        let mut world = World::default();
        let mut outer = &mut world.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = &mut world.objects[1];
        inner.material.ambient = 1.0;
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let resulting_colour = inner.material.colour;
        assert_eq!(Comps::colour_at(world, ray), resulting_colour);
    }
}
