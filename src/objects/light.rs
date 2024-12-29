use super::Material;
use crate::collections::{Colour, Point, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Colour,
}

impl PointLight {
    pub fn new(position: Point, intensity: Colour) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

// This function desperately needs to be refactored; 5 parameters is too many
// especially when some of the parameters can already be determined from the
// others and can be computed in the body instead.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pointlight() {
        let position = Point::new(1.0, 1.0, 1.0);
        let intensity = Colour::new(0.0, 0.0, 0.0);
        let pointlight = PointLight::new(position, intensity);
        let resulting_pointlight = PointLight {
            position,
            intensity,
        };
        assert_eq!(pointlight, resulting_pointlight)
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
            lighting(material, pointlight, position, eyev, normalv),
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
            lighting(material, pointlight, position, eyev, normalv),
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
            lighting(material, pointlight, position, eyev, normalv),
            resulting_colour
        );
    }
}
