use std::io::Write;

use crate::collections::Colour;
use crate::utils::filehandler;

const PPM_HEADER: &str = "P3";
const PIXEL_MAX: u64 = 255;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Width(pub usize);
pub struct Height(pub usize);

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
    OutOfBounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    size: Size,
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Size {
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(Width(width): Width, Height(height): Height) -> Canvas {
        let mut canvas: Vec<Vec<Pixel>> = Vec::with_capacity(height);
        for _row in 0..height {
            let mut row_pixels = Vec::with_capacity(width);
            for _column in 0..width {
                row_pixels.push(Pixel::paint(Colour::new(0.0, 0.0, 0.0)));
            }
            canvas.push(row_pixels);
        }
        Canvas {
            size: Size { width, height },
            pixels: canvas,
        }
    }

    pub fn paint_colour(
        &mut self,
        column: usize,
        row: usize,
        colour: Colour,
    ) -> Result<(), WriteError> {
        match (column, row) {
            (column, row) if column > self.size.width || row > self.size.height => {
                return Err(WriteError::OutOfBounds)
            }
            _ => (),
        };

        self.pixels[row][column] = Pixel::paint(colour);
        Ok(())
    }

    pub fn write_to_ppm(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        writeln!(&mut buffer, "{}", PPM_HEADER)?;
        writeln!(&mut buffer, "{} {}", self.size.width, self.size.height)?;
        writeln!(&mut buffer, "{}", PIXEL_MAX)?;
        for row in &self.pixels {
            let mut row_buffer = String::new();
            for pixel in row {
                let colour_values: Vec<String> = vec![pixel.red, pixel.green, pixel.blue]
                    .iter()
                    .map(|cval| cval.to_string())
                    .collect();
                for colour_value in colour_values {
                    if row_buffer.len() + colour_value.len() + 1 > 70 {
                        writeln!(buffer, "{}", row_buffer.trim())?;
                        row_buffer = String::new();
                    }
                    row_buffer.push_str(&colour_value[..]);
                    row_buffer.push(' ');
                }
            }
            writeln!(buffer, "{}", row_buffer.trim())?;
        }
        Ok(buffer)
    }

    pub fn output_to_ppm(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = self.write_to_ppm()?;

        filehandler::write_to_file(&buffer, output_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

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
        let canvas = Canvas::new(Width(1), Height(2));
        let black_pixel = Pixel::paint(Colour::new(0.0, 0.0, 0.0));
        let resulting_canvas = vec![vec![black_pixel], vec![black_pixel]];
        assert_eq!(
            canvas,
            Canvas {
                size: Size {
                    width: 1,
                    height: 2,
                },
                pixels: resulting_canvas,
            }
        );
    }

    #[test]
    fn create_and_paint_canvas() {
        let mut canvas = Canvas::new(Width(2), Height(3));
        let black_pixel = Pixel::paint(Colour::new(0.0, 0.0, 0.0));
        let grey_colour = Colour::new(0.5, 0.5, 0.5);
        let grey_pixel = Pixel::paint(Colour::new(0.5, 0.5, 0.5));
        canvas.paint_colour(0, 1, grey_colour).unwrap();
        let resulting_canvas = vec![
            vec![black_pixel, black_pixel],
            vec![grey_pixel, black_pixel],
            vec![black_pixel, black_pixel],
        ];
        assert_eq!(
            canvas,
            Canvas {
                size: Size {
                    width: 2,
                    height: 3,
                },
                pixels: resulting_canvas,
            }
        );
    }

    #[test]
    fn write_ppm_small_canvas() {
        let mut canvas = Canvas::new(Width(2), Height(2));
        canvas
            .paint_colour(0, 0, Colour::new(1.0, 1.0, 1.0))
            .unwrap();
        canvas
            .paint_colour(1, 1, Colour::new(0.5, 0.5, 0.5))
            .unwrap();
        let output_buffer = b"P3\n2 2\n255\n255 255 255 0 0 0\n0 0 0 128 128 128\n".to_vec();
        let written_buffer = canvas.write_to_ppm().unwrap();
        assert_eq!(written_buffer, output_buffer);
    }

    #[test]
    fn write_ppm_large_canvas() {
        let mut canvas = Canvas::new(Width(10), Height(2));
        for pixel in 0..10 {
            canvas
                .paint_colour(pixel, 0, Colour::new(1.0, 1.0, 1.0))
                .unwrap();
        }
        let output_buffer = b"P3\n10 2\n255\n255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255\n255 255 255 255 255 255 255 255 255 255 255 255 255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n".to_vec();
        let written_buffer = canvas.write_to_ppm().unwrap();
        assert_eq!(written_buffer, output_buffer);
    }

    #[test]
    #[ignore]
    fn output_canvas_to_ppm() {
        let mut canvas = Canvas::new(Width(2), Height(2));
        canvas
            .paint_colour(0, 0, Colour::new(1.0, 1.0, 1.0))
            .unwrap();
        canvas
            .paint_colour(1, 1, Colour::new(0.5, 0.5, 0.5))
            .unwrap();
        let output_buffer = b"P3\n2 2\n255\n255 255 255 0 0 0\n0 0 0 128 128 128\n".to_vec();

        canvas.output_to_ppm("test.ppm").unwrap();

        let mut read_buffer = Vec::new();
        File::open("test.ppm")
            .unwrap()
            .read_to_end(&mut read_buffer)
            .unwrap();

        assert_eq!(read_buffer, output_buffer);

        // cleanup
        std::fs::remove_file("test.ppm").unwrap();
    }
}