use super::Native;
use crate::collections::{Angle, Point};
use crate::objects::{Ray, Transform, Transformable};
use crate::scenes::raygen;
use crate::scenes::raygen::{RayGenerator, TaggedPixel, TaggedRay};
use crate::scenes::Orientation;
use crate::utils::floats::EPSILON;

pub struct Agss {
    render_scale: f64,
    native: Native,
}

impl Agss {
    pub fn new(
        hsize: usize,
        vsize: usize,
        fov: Angle,
        orientation: Orientation,
        render_scale: f64,
    ) -> Agss {
        let native = Native::new(hsize, vsize, fov, orientation);
        Agss {
            render_scale,
            native,
        }
    }

    pub fn hsize(&self) -> usize {
        self.native.hsize()
    }

    pub fn vsize(&self) -> usize {
        self.native.vsize()
    }

    pub fn fov(&self) -> Angle {
        self.native.fov()
    }

    pub fn frame_transformation(&self) -> &Transform {
        self.native.frame_transformation()
    }

    pub fn half_height(&self) -> f64 {
        self.native.half_height()
    }

    pub fn half_width(&self) -> f64 {
        self.native.half_width()
    }

    pub fn pixel_size(&self) -> f64 {
        self.native.pixel_size()
    }

    pub fn render_scale(&self) -> f64 {
        self.render_scale
    }
}

impl IntoIterator for Agss {
    type Item = TaggedRay;
    type IntoIter = AgssIterator;

    fn into_iter(self) -> Self::IntoIter {
        let render_scale = self.render_scale();
        let hsize = f64::ceil(self.hsize() as f64 * render_scale) as usize;
        let vsize = f64::ceil(self.vsize() as f64 * render_scale) as usize;
        let pixel_iterator = Box::new(
            (0..hsize).flat_map(move |pos_x| std::iter::repeat(pos_x).take(vsize).zip(0..vsize)),
        );

        AgssIterator {
            pixel_iterator,
            render_scale,
            native: self.native,
        }
    }
}

impl RayGenerator for Agss {
    fn canvas_size(&self) -> (usize, usize) {
        (self.hsize(), self.vsize())
    }
}

pub struct AgssIterator {
    pixel_iterator: Box<dyn Iterator<Item = (usize, usize)>>,
    render_scale: f64,
    native: Native,
}

impl Iterator for AgssIterator {
    type Item = TaggedRay;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pixel_iterator.next() {
            Some((pos_x, pos_y)) => {
                // compute ray target coordinate offset from origin (native res)
                let subpixel_size = self.native.pixel_size() / self.render_scale;
                let (offset_x, offset_y) = raygen::pixel_offset_from_centre_target(
                    pos_x,
                    pos_y,
                    subpixel_size,
                    self.native.half_width(),
                    self.native.half_height(),
                );
                let ray = raygen::generate_normalised_ray(
                    Point::zero(),
                    Point::new(offset_x, offset_y, -1.0),
                    &self.native.frame_transformation().invert(),
                );

                // compute subpixel-pixel boundary intersections
                // (x0, y0) and (x1, y1) denote the subpixel's boundaries in the pixel frame
                let corner_0 = [
                    pos_x as f64 / self.render_scale,
                    pos_y as f64 / self.render_scale,
                ];
                // edge alignment strategy: truncation
                let corner_1 = [
                    f64::min(
                        (pos_x as f64 + 1.0) / self.render_scale,
                        self.native.hsize() as f64,
                    ),
                    f64::min(
                        (pos_y as f64 + 1.0) / self.render_scale,
                        self.native.vsize() as f64,
                    ),
                ];

                let mut tagged_pixels: Vec<TaggedPixel> = vec![TaggedPixel::new(
                    [
                        f64::floor(corner_0[0]) as usize,
                        f64::floor(corner_0[1]) as usize,
                    ],
                    (corner_1[0] - corner_0[0]) * (corner_1[1] - corner_0[1]),
                )];

                for axis_index in 0_usize..=1_usize {
                    if f64::ceil(corner_0[axis_index] + EPSILON)
                        == f64::floor(corner_1[axis_index] - EPSILON)
                    {
                        let mut new_tagged_pixels =
                            Vec::with_capacity(2_usize.pow(axis_index as u32 + 1));

                        for tagged_pixel in tagged_pixels {
                            new_tagged_pixels.append(&mut raygen::section_pixel(
                                tagged_pixel,
                                corner_0[axis_index],
                                corner_1[axis_index],
                                axis_index,
                            ));
                        }
                        tagged_pixels = new_tagged_pixels;
                    }
                }

                let tagged_ray = TaggedRay::new(ray, tagged_pixels);
                Some(tagged_ray)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::Vector;
    use crate::utils::approx_eq;

    use super::*;

    #[test]
    fn integer_render_scale_centre_of_screen() {
        let canvas = Agss::new(
            7,
            7,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
            3.0,
        );
        let tagged_ray = canvas.into_iter().skip(21 * 10 + 10).next().unwrap();
        let casted_ray = tagged_ray.ray();
        let resulting_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -1.0));
        approx_eq!(casted_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(casted_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(casted_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(casted_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(casted_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(casted_ray.direction.z, resulting_ray.direction.z);

        let pixels = tagged_ray.pixels();
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0].index(), [3, 3]);
        approx_eq!(pixels[0].blend_weight(), 0.111111);
    }

    #[test]
    fn noninteger_render_scale_edge_aligned_centre_of_screen() {
        let canvas = Agss::new(
            14,
            14,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
            1.5,
        );
        let tagged_ray = canvas.into_iter().skip(21 * 10 + 10).next().unwrap();
        let casted_ray = tagged_ray.ray();
        let resulting_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -1.0));
        approx_eq!(casted_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(casted_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(casted_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(casted_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(casted_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(casted_ray.direction.z, resulting_ray.direction.z);

        let pixels = tagged_ray.pixels();
        assert_eq!(pixels.len(), 4);
        assert_eq!(pixels[0].index(), [6, 6]);
        approx_eq!(pixels[0].blend_weight(), 0.111111);
        assert_eq!(pixels[1].index(), [6, 7]);
        approx_eq!(pixels[1].blend_weight(), 0.111111);
        assert_eq!(pixels[2].index(), [7, 6]);
        approx_eq!(pixels[2].blend_weight(), 0.111111);
        assert_eq!(pixels[3].index(), [7, 7]);
        approx_eq!(pixels[3].blend_weight(), 0.111111);
    }

    #[test]
    fn noninteger_render_scale_edge_misaligned() {
        let canvas = Agss::new(
            10,
            10,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
            1.0 + (1.0 / 3.0),
        );

        let mut ray_generator = canvas.into_iter().skip(14 + 13);

        // edge
        let tagged_ray = ray_generator.next().unwrap();
        let pixels = tagged_ray.pixels();
        assert_eq!(pixels.len(), 2);
        assert_eq!(pixels[0].index(), [0, 9]);
        approx_eq!(pixels[0].blend_weight(), 0.06250);
        assert_eq!(pixels[1].index(), [1, 9]);
        approx_eq!(pixels[1].blend_weight(), 0.12500);

        // corner
        let tagged_ray = ray_generator.last().unwrap();
        let pixels = tagged_ray.pixels();
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0].index(), [9, 9]);
        approx_eq!(pixels[0].blend_weight(), 0.06250);
    }
}
