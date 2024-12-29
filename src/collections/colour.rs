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
}
