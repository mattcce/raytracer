use crate::collections::{Colour, Point, Vector};

use super::Material;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub position: Point,
    pub intensity: Colour,
}

impl Light {
    pub fn new(position: Point, intensity: Colour) -> Light {
        Light {
            position,
            intensity,
        }
    }

    pub(crate) fn shade_phong(
        &self,
        material: &Material,
        target: Point,
        eyev: Vector,
        normal: Vector,
        shadowed: bool,
    ) -> Colour {
        let effective_colour = material.pattern.colour_at(target) * self.intensity;
        let lightv = (self.position - target).normalise();
        let ambient = effective_colour * material.ambient;
        if shadowed {
            return ambient;
        }

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
                specular = self.intensity * material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::floats::approx_eq;

    use super::*;

    #[test]
    fn eye_directly_between_light_and_surface() {
        let material = Material::preset();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(1.9, 1.9, 1.9);
        assert_eq!(
            light.shade_phong(&material, position, eyev, normal, false),
            resulting_colour
        );
    }

    #[test]
    fn eye_between_light_and_surface_eye_offset_45_degrees() {
        let material = Material::preset();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(1.0, 1.0, 1.0);
        assert_eq!(
            light.shade_phong(&material, position, eyev, normal, false),
            resulting_colour
        );
    }

    #[test]
    fn eye_between_light_and_surface_light_offset_45_degrees() {
        let material = Material::preset();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let colour = light.shade_phong(&material, position, eyev, normal, false);
        let resulting_colour = Colour::new(0.736396, 0.736396, 0.736396);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn eye_in_path_of_reflection_vector() {
        let material = Material::preset();
        let position = Point::zero();
        let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let colour = light.shade_phong(&material, position, eyev, normal, false);
        let resulting_colour = Colour::new(1.636396, 1.636396, 1.636396);
        approx_eq!(colour.red, resulting_colour.red);
        approx_eq!(colour.green, resulting_colour.green);
        approx_eq!(colour.blue, resulting_colour.blue);
    }

    #[test]
    fn light_behind_surface() {
        let material = Material::preset();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, 10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(0.1, 0.1, 0.1);
        assert_eq!(
            light.shade_phong(&material, position, eyev, normal, false),
            resulting_colour
        );
    }

    #[test]
    fn light_in_shadow() {
        let material = Material::preset();
        let position = Point::zero();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let resulting_colour = Colour::new(0.1, 0.1, 0.1);
        assert_eq!(
            light.shade_phong(&material, position, eyev, normal, true),
            resulting_colour
        );
    }
}
