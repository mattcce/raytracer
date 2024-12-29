const PPM_HEADER: &str = "P3\n";
const COLOUR_MAX: u64 = 255;

struct Canvas {
    pixels: Vec<Vec<Colour>>,
}
