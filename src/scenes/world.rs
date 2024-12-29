use crate::collections::*;
use crate::objects::*;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere {
            transform: Transform::new(TransformKind::Identity),
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
        };
        let s2 = Sphere {
            transform: Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            material: Material::default(),
        };
        let light = PointLight::new(Point::new(-10.0, -10.0, -10.0), Colour::new(1.0, 1.0, 1.0));

        World {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}

impl Intersectable<World> for Ray {
    fn intersect<'a>(&'a self, object: &'a World) -> Intersections<'a> {
        let mut intersections = Intersections::new();
        for object in &object.objects {
            for intersect in self.intersect(object).0 {
                intersections.add(intersect);
            }
        }
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world() {
        let world = World::new();
        let resulting_world = World {
            objects: vec![],
            lights: vec![],
        };
        assert_eq!(world, resulting_world);
    }

    #[test]
    fn create_default_world() {
        let default_world = World::default();
        let s1 = Sphere {
            transform: Transform::new(TransformKind::Identity),
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::default()
            },
        };
        let s2 = Sphere {
            transform: Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)),
            material: Material::default(),
        };
        let light = PointLight::new(Point::new(-10.0, -10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
        assert_eq!(default_world, resulting_world);
    }

    #[test]
    fn intersect_ray_with_world() {
        let s1 = Sphere {
            ..Sphere::default()
        };

        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let world = World::default();
        let intersections = ray.intersect(&world);
        println!("{:?}", intersections.0);
        assert_eq!(intersections.0.len(), 4);
        assert_eq!(intersections[0].t(), 4.0);
        assert_eq!(intersections[1].t(), 4.5);
        assert_eq!(intersections[2].t(), 5.5);
        assert_eq!(intersections[3].t(), 6.0);
    }
}
