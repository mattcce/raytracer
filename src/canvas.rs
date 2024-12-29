const PPM_HEADER: &str = "P3\n";
const COLOUR_MAX: u64 = 255;

struct CanvasSize {
    width: u64,
    height: u64,
}

impl CanvasSize {
    pub fn new(width: u64, height: u64) -> CanvasSize {
        CanvasSize { width, height }
    }
}

struct Canvas {
    pixels: Vec<Vec<Colour>>,
}
