use crate::collections::Colour;

const PPM_HEADER: &str = "P3\n";
const COLOUR_MAX: u64 = 255;

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

#[derive(Debug)]
pub enum WriteError {
    NegativeCoords,
    OutOfBounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    size: CanvasSize,
    pixels: Vec<Vec<Colour>>,
}

impl Canvas {
    pub fn new(size: CanvasSize) -> Canvas {
        let mut canvas: Vec<Vec<Colour>> = Vec::with_capacity(size.height as usize);
        for _row in 0..size.height {
            let mut row_pixels = Vec::with_capacity(size.width as usize);
            for _column in 0..size.width {
                row_pixels.push(Colour::new(0.0, 0.0, 0.0));
            }
            canvas.push(row_pixels);
        }
        Canvas {
            size,
            pixels: canvas,
        }
    }

    pub fn draw_pixel(&mut self, column: i128, row: i128, pixel: Colour) -> Result<(), WriteError> {
        match (column, row) {
            (column, row) if column < 0 || row < 0 => return Err(WriteError::NegativeCoords),
            (column, row) if column > self.size.width as i128 || row > self.size.height as i128 => {
                return Err(WriteError::OutOfBounds)
            }
            _ => (),
        };

        self.pixels[column as usize][row as usize] = pixel;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let size = CanvasSize::new(1, 2);
        let canvas = Canvas::new(size);
        let black_pixel = Colour::new(0.0, 0.0, 0.0);
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
    fn create_and_modify_canvas() {
        let size = CanvasSize::new(2, 2);
        let mut canvas = Canvas::new(size);
        let black_pixel = Colour::new(0.0, 0.0, 0.0);
        let white_pixel = Colour::new(1.0, 1.0, 1.0);
        canvas.draw_pixel(1, 1, white_pixel).unwrap();
        let resulting_canvas = vec![
            vec![black_pixel, black_pixel],
            vec![black_pixel, white_pixel],
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
