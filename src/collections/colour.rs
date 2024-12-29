use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    pub(super) red: f64,
    pub(super) green: f64,
    pub(super) blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Colour {
        Colour { red, green, blue }
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, other: Colour) -> Self::Output {
        Colour {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub<Colour> for Colour {
    type Output = Colour;

    fn sub(self, other: Colour) -> Self::Output {
        Colour {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, other: Colour) -> Self::Output {
        Colour {
            red: self * other.red,
            green: self * other.green,
            blue: self * other.blue,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, other: f64) -> Self::Output {
        other * self
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, other: Colour) -> Self::Output {
        Colour {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_colour() {
        let colour_new = Colour::new(0.5, 0.5, 0.5);
        let colour_direct = Colour {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        };
        assert_eq!(colour_new, colour_direct);
    }

    #[test]
    fn add_two_colours() {
        let colour1 = Colour::new(0.9, 0.6, 0.7);
        let colour2 = Colour::new(0.7, 0.1, 1.0);
        let resulting_colour = Colour::new(1.6, 0.7, 1.7);
        assert_eq!(colour1 + colour2, resulting_colour);
    }

    #[test]
    fn sub_two_colours() {
        let colour1 = Colour::new(1.0, 0.6, 1.7);
        let colour2 = Colour::new(0.0, 0.1, 1.0);
        let resulting_colour = Colour::new(1.0, 0.5, 0.7);
        assert_eq!(colour1 - colour2, resulting_colour);
    }

    #[test]
    fn mul_colour_by_scalar() {
        let colour = Colour::new(0.2, 0.3, 0.4);
        let scalar = 2.0_f64;
        let resulting_colour = Colour::new(0.4, 0.6, 0.8);
        assert_eq!(colour * scalar, resulting_colour);
    }

    #[test]
    fn mul_scalar_by_colour() {
        let scalar = 2.0_f64;
        let colour = Colour::new(0.2, 0.3, 0.4);
        let resulting_colour = Colour::new(0.4, 0.6, 0.8);
        assert_eq!(scalar * colour, resulting_colour);
    }

    #[test]
    fn mul_two_colours() {
        let colour1 = Colour::new(1.0, 0.2, 0.5);
        let colour2 = Colour::new(0.9, 1.0, 0.5);
        let resulting_colour = Colour::new(0.9, 0.2, 0.25);
        assert_eq!(colour1 * colour2, resulting_colour);
    }
}
