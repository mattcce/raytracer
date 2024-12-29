use std::f64::consts::PI as MATH_PI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Angle {
    degrees: Option<f64>,
    radians: Option<f64>,
}

impl Angle {
    pub fn from_degrees(deg: f64) -> Angle {
        Angle {
            degrees: Some(deg),
            radians: None,
        }
    }

    pub fn from_radians(rad: f64) -> Angle {
        Angle {
            degrees: None,
            radians: Some(rad),
        }
    }

    pub fn degrees(&mut self) -> f64 {
        match self.degrees {
            Some(deg) => deg,
            None => {
                self.degrees = Some(self.radians.unwrap() * (180.0 / MATH_PI));
                self.degrees.unwrap()
            }
        }
    }

    pub fn radians(&mut self) -> f64 {
        match self.radians {
            Some(rad) => rad,
            None => {
                self.radians = Some(self.degrees.unwrap() * (MATH_PI / 180.0));
                self.radians.unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_angle_from_degrees() {
        let angle = Angle::from_degrees(145.0);
        let resulting_angle = Angle {
            degrees: Some(145.0),
            radians: None,
        };
        assert_eq!(angle, resulting_angle);
    }

    #[test]
    fn create_angle_from_radians() {
        let angle = Angle::from_radians(2.0);
        let resulting_angle = Angle {
            degrees: None,
            radians: Some(2.0),
        };
        assert_eq!(angle, resulting_angle);
    }

    #[test]
    fn access_angle_from_degrees() {
        let mut angle = Angle::from_degrees(145.0);
        let resulting_angle = Angle {
            degrees: Some(145.0),
            radians: Some(145.0 * (MATH_PI / 180.0)),
        };
        angle.radians();
        assert_eq!(angle, resulting_angle);
    }

    #[test]
    fn access_angle_from_radians() {
        let mut angle = Angle::from_radians(2.0);
        let resulting_angle = Angle {
            degrees: Some(2.0 * (180.0 / MATH_PI)),
            radians: Some(2.0),
        };
        angle.degrees();
        assert_eq!(angle, resulting_angle);
    }
}
