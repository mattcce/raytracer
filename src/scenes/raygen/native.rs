use crate::collections::{Angle, Point};
use crate::objects::{Ray, Transform, Transformable};
use crate::scenes::raygen::{RayGenerator, TaggedPixel, TaggedRay};
use crate::scenes::Orientation;

pub struct Native {
    hsize: usize,
    vsize: usize,
    fov: Angle,
    frame_transformation: Transform,
    half_height: f64,
    half_width: f64,
    pixel_size: f64,
}

impl Native {
    pub fn new(
        hsize: usize,
        vsize: usize,
        mut fov: Angle,
        Orientation(frame_transformation): Orientation,
    ) -> Native {
        let half_view = (fov.radians() / 2.0).tan();

        let half_width;
        let half_height;
        match hsize as f64 / vsize as f64 {
            aspect_ratio if aspect_ratio >= 1.0 => {
                half_width = half_view;
                half_height = half_view / aspect_ratio;
            }
            aspect_ratio if aspect_ratio < 1.0 => {
                half_width = half_view * aspect_ratio;
                half_height = half_view;
            }
            _ => panic!(),
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Native {
            hsize,
            vsize,
            fov,
            frame_transformation,
            half_height,
            half_width,
            pixel_size,
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn fov(&self) -> Angle {
        self.fov
    }

    pub fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    pub fn half_height(&self) -> f64 {
        self.half_height
    }

    pub fn half_width(&self) -> f64 {
        self.half_width
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }
}

impl IntoIterator for Native {
    type Item = TaggedRay;
    type IntoIter = NativeIterator;

    fn into_iter(self) -> Self::IntoIter {
        let hsize = self.hsize();
        let vsize = self.vsize();
        let pixel_iterator = Box::new(
            (0..hsize).flat_map(move |pos_x| std::iter::repeat(pos_x).take(vsize).zip(0..vsize)),
        );

        NativeIterator {
            pixel_iterator,
            native: self,
        }
    }
}

impl RayGenerator for Native {
    fn canvas_size(&self) -> (usize, usize) {
        (self.hsize, self.vsize)
    }
}

pub struct NativeIterator {
    pixel_iterator: Box<dyn Iterator<Item = (usize, usize)>>,
    native: Native,
}

impl Iterator for NativeIterator {
    type Item = TaggedRay;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pixel_iterator.next() {
            Some((pos_x, pos_y)) => {
                let offset_x = (pos_x as f64 + 0.5) * self.native.pixel_size();
                let offset_y = (pos_y as f64 + 0.5) * self.native.pixel_size();
                let world_x = self.native.half_width() - offset_x;
                let world_y = self.native.half_height() - offset_y;
                let inverse_transform = self.native.frame_transformation().invert();
                let pixel = Point::new(world_x, world_y, -1.0).transform(&inverse_transform);
                let origin = Point::new(0.0, 0.0, 0.0).transform(&inverse_transform);
                let direction = (pixel - origin).normalise();
                let ray = Ray::new(origin, direction);
                let tagged_pixel = TaggedPixel::new([pos_x, pos_y], 1.0);
                let tagged_ray = TaggedRay::new(ray, vec![tagged_pixel]);
                Some(tagged_ray)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::*;
    use crate::objects::*;
    use crate::scenes::Orientation;
    use crate::utils::approx_eq;

    use super::*;

    #[test]
    fn pixel_size() {
        let horizontal_canvas = Native::new(
            200,
            125,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
        );
        let vertical_canvas = Native::new(
            125,
            200,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
        );
        approx_eq!(horizontal_canvas.pixel_size, 0.01);
        approx_eq!(vertical_canvas.pixel_size, 0.01);
    }

    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn ray_through_centre_of_camera_view() {
        let native = Native::new(
            201,
            101,
            Angle::from_radians(FRAC_PI_2),
            Orientation::default(),
        );
        let tagged_ray = native.into_iter().skip(101 * 100 + 50).next().unwrap(); // next ray for pixel [100, 50]
        println!("{:?}", tagged_ray.pixels());
        let casted_ray = tagged_ray.ray();
        let resulting_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -1.0));
        approx_eq!(casted_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(casted_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(casted_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(casted_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(casted_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(casted_ray.direction.z, resulting_ray.direction.z);
    }

    #[test]
    fn ray_through_corner_of_camera_view() {
        let native = Native::new(
            201,
            101,
            Angle::from_radians(FRAC_PI_2),
            Orientation::default(),
        );
        let casted_ray = native.into_iter().next().unwrap().ray();
        let resulting_ray = Ray::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.665186, 0.332593, -0.668512),
        );
        approx_eq!(casted_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(casted_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(casted_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(casted_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(casted_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(casted_ray.direction.z, resulting_ray.direction.z);
    }

    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn ray_with_transformed_camera() {
        let transform = Transform::from(vec![
            TransformKind::Translate(0.0, -2.0, 5.0),
            TransformKind::Rotate(crate::prelude::Axis::Y, Angle::from_radians(FRAC_PI_4)),
        ]);
        let native = Native::new(
            201,
            101,
            Angle::from_radians(FRAC_PI_2),
            Orientation::default().transform(&transform),
        );
        let casted_ray = native
            .into_iter()
            .skip(101 * 100 + 50)
            .next()
            .unwrap()
            .ray();
        let resulting_ray = Ray::new(
            Point::new(0.0, 2.0, -5.0),
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0),
        );
        approx_eq!(casted_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(casted_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(casted_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(casted_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(casted_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(casted_ray.direction.z, resulting_ray.direction.z);
    }
}
