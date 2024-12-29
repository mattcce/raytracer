use crate::collections::Colour;

const PPM_HEADER: &str = "P3\n";
const PIXEL_MAX: u64 = 255;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasSize {
    width: u64,
    height: u64,
}

impl CanvasSize {
    pub fn new(width: u64, height: u64) -> CanvasSize {
        CanvasSize { width, height }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pixel {
    red: u64,
    green: u64,
    blue: u64,
}

impl Pixel {
    pub fn paint(colour: Colour) -> Pixel {
        Pixel {
            red: match colour.red {
                x if x > 1.0 => PIXEL_MAX,
                x if x < 0.0 => 0,
                x => (x * PIXEL_MAX as f64).round() as u64,
            },
            green: match colour.green {
                x if x > 1.0 => PIXEL_MAX,
                x if x < 0.0 => 0,
                x => (x * PIXEL_MAX as f64).round() as u64,
            },
            blue: match colour.blue {
                x if x > 1.0 => PIXEL_MAX,
                x if x < 0.0 => 0,
                x => (x * PIXEL_MAX as f64).round() as u64,
            },
        }
    }
}

#[derive(Debug)]
pub enum WriteError {
    NegativeCoords,
    OutOfBounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    size: CanvasSize,
    pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(size: CanvasSize) -> Canvas {
        let mut canvas: Vec<Vec<Pixel>> = Vec::with_capacity(size.height as usize);
        for _row in 0..size.height {
            let mut row_pixels = Vec::with_capacity(size.width as usize);
            for _column in 0..size.width {
                row_pixels.push(Pixel::paint(Colour::new(0.0, 0.0, 0.0)));
            }
            canvas.push(row_pixels);
        }
        Canvas {
            size,
            pixels: canvas,
        }
    }

    pub fn paint_colour(
        &mut self,
        column: i128,
        row: i128,
        colour: Colour,
    ) -> Result<(), WriteError> {
        match (column, row) {
            (column, row) if column < 0 || row < 0 => return Err(WriteError::NegativeCoords),
            (column, row) if column > self.size.width as i128 || row > self.size.height as i128 => {
                return Err(WriteError::OutOfBounds)
            }
            _ => (),
        };

        self.pixels[column as usize][row as usize] = Pixel::paint(colour);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pixel() {
        let colour = Colour::new(0.25, 0.3, 0.75);
        let resulting_pixel = Pixel {
            red: 64,
            green: 77,
            blue: 191,
        };
        assert_eq!(Pixel::paint(colour), resulting_pixel)
    }

    #[test]
    fn create_canvas() {
        let size = CanvasSize::new(1, 2);
        let canvas = Canvas::new(size);
        let black_pixel = Pixel::paint(Colour::new(0.0, 0.0, 0.0));
        let resulting_canvas = vec![vec![black_pixel], vec![black_pixel]];
        assert_eq!(
            canvas,
            Canvas {
                size: size,
                pixels: resulting_canvas,
            }
        );
    }

    #[test]
    fn create_and_paint_canvas() {
        let size = CanvasSize::new(2, 2);
        let mut canvas = Canvas::new(size);
        let black_pixel = Pixel::paint(Colour::new(0.0, 0.0, 0.0));
        let grey_colour = Colour::new(0.5, 0.5, 0.5);
        let grey_pixel = Pixel::paint(Colour::new(0.5, 0.5, 0.5));
        canvas.paint_colour(1, 1, grey_colour).unwrap();
        let resulting_canvas = vec![
            vec![black_pixel, black_pixel],
            vec![black_pixel, grey_pixel],
        ];
        assert_eq!(
            canvas,
            Canvas {
                size: size,
                pixels: resulting_canvas,
            }
        );
    }
}
