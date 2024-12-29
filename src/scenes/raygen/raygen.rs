use crate::collections::Point;
use crate::objects::{Ray, Transform, Transformable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaggedPixel {
    pub index: [usize; 2],
    pub blend_weight: f64,
}

impl TaggedPixel {
    pub fn new(index: [usize; 2], blend_weight: f64) -> TaggedPixel {
        TaggedPixel {
            index,
            blend_weight,
        }
    }

    pub fn index(&self) -> [usize; 2] {
        self.index
    }

    pub fn blend_weight(&self) -> f64 {
        self.blend_weight
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaggedRay {
    pub ray: Ray,
    pub pixels: Vec<TaggedPixel>,
}

impl TaggedRay {
    pub fn new(ray: Ray, pixels: Vec<TaggedPixel>) -> TaggedRay {
        TaggedRay { ray, pixels }
    }

    pub fn ray(&self) -> Ray {
        self.ray
    }

    pub fn pixels(&self) -> &Vec<TaggedPixel> {
        &self.pixels
    }
}

pub trait RayGenerator: IntoIterator<Item = TaggedRay> {
    fn canvas_size(&self) -> (usize, usize);
}

pub fn pixel_offset_from_centre_target(
    pixel_pos_x: usize,
    pixel_pos_y: usize,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
) -> (f64, f64) {
    (
        half_width - ((pixel_pos_x as f64 + 0.5) * pixel_size),
        half_height - ((pixel_pos_y as f64 + 0.5) * pixel_size),
    )
}

pub fn subpixel_to_pixel_frame([subpixel_x, subpixel_y]: [f64; 2], render_scale: f64) -> [f64; 2] {
    [(subpixel_x / render_scale), (subpixel_y / render_scale)]
}

pub fn generate_normalised_ray(
    ray_origin: Point,
    ray_target: Point,
    frame_transformation: &Transform,
) -> Ray {
    let transformed_ray_origin = ray_origin.transform(frame_transformation);
    let transformed_ray_target = ray_target.transform(frame_transformation);
    let ray_direction = (transformed_ray_target - transformed_ray_origin).normalise();
    Ray::new(transformed_ray_origin, ray_direction)
}

pub fn section_pixel(
    tagged_pixel: TaggedPixel,
    coordinate_0: f64,
    coordinate_1: f64,
    axis_index: usize,
) -> Vec<TaggedPixel> {
    assert!(coordinate_0 < ((tagged_pixel.index()[axis_index] + 1) as f64));
    assert!(((tagged_pixel.index()[axis_index] + 1) as f64) < coordinate_1);

    let tagged_pixel_index_1 = tagged_pixel.index();
    let boundary = f64::ceil(coordinate_0);

    let old_length = coordinate_1 - coordinate_0;

    let old_blend_weight = tagged_pixel.blend_weight();
    let blend_weight_1_ratio = (boundary - coordinate_0) / old_length;
    let tagged_pixel_1 = TaggedPixel::new(
        tagged_pixel_index_1,
        old_blend_weight * blend_weight_1_ratio,
    );

    let mut tagged_pixel_index_2 = tagged_pixel_index_1.clone();
    tagged_pixel_index_2[axis_index] += 1;
    let blend_weight_2_ratio = (coordinate_1 - boundary) / old_length;
    let tagged_pixel_2 = TaggedPixel::new(
        tagged_pixel_index_2,
        old_blend_weight * blend_weight_2_ratio,
    );
    vec![tagged_pixel_1, tagged_pixel_2]
}

#[cfg(test)]
mod tests {
    use crate::utils::approx_eq;

    use super::*;

    #[test]
    fn centre_pixel_offset() {
        let pixel_pos_x = 10;
        let pixel_pos_y = 10;
        let pixel_size = 0.01;
        let half_width = 0.105;
        let half_height = 0.105;
        let pixel_offset = pixel_offset_from_centre_target(
            pixel_pos_x,
            pixel_pos_y,
            pixel_size,
            half_width,
            half_height,
        );
        approx_eq!(pixel_offset.0, 0.0);
        approx_eq!(pixel_offset.1, 0.0);
    }

    #[test]
    fn corner_pixel_offset() {
        let pixel_pos_x = 0;
        let pixel_pos_y = 19;
        let pixel_size = 0.01;
        let half_width = 0.1;
        let half_height = 0.1;
        let pixel_offset = pixel_offset_from_centre_target(
            pixel_pos_x,
            pixel_pos_y,
            pixel_size,
            half_width,
            half_height,
        );
        approx_eq!(pixel_offset.0, 0.095);
        approx_eq!(pixel_offset.1, -0.095);
    }

    #[test]
    fn section_pixels() {
        let tagged_pixel = TaggedPixel::new([0, 1], 0.5);
        let sectioned_pixels = section_pixel(tagged_pixel, 0.75, 1.25, 0);
        assert_eq!(sectioned_pixels.len(), 2);
        assert_eq!(sectioned_pixels[0].index(), [0, 1]);
        assert_eq!(sectioned_pixels[0].blend_weight(), 0.25);
        assert_eq!(sectioned_pixels[1].index(), [1, 1]);
        assert_eq!(sectioned_pixels[1].blend_weight(), 0.25);
    }
}
