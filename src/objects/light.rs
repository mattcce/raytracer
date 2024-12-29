use crate::collections::{Colour, Point};

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
}
