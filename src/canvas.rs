use crate::collections::Colour;

const PPM_HEADER: &str = "P3\n";
const COLOUR_MAX: u64 = 255;

#[derive(Clone, Copy, Debug, PartialEq)]
struct CanvasSize {
    width: u64,
    height: u64,
}

impl CanvasSize {
    pub fn new(width: u64, height: u64) -> CanvasSize {
        CanvasSize { width, height }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Canvas {
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
}
