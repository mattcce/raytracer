use crate::collections::{Angle, Matrix, Point, Vector};
use crate::objects::{Ray, Transform, TransformKind, Transformable};
use crate::scenes::WriteError;
use crate::scenes::{Canvas, Height, Width, World};

#[derive(Clone, Debug, PartialEq)]
pub struct Orientation(Transform);

impl Orientation {
    pub fn new(from: Point, to: Point, up: Vector) -> Orientation {
        Orientation(Orientation::view_transform(from, to, up))
    }

    fn view_transform(from: Point, to: Point, up: Vector) -> Transform {
        let forward = (to - from).normalise();
        let upn = up.normalise();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);

        let orientation = Matrix::from(&vec![
            vec![left.x, left.y, left.z, 0.0],
            vec![true_up.x, true_up.y, true_up.z, 0.0],
            vec![-forward.x, -forward.y, -forward.z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        Transform::new(TransformKind::Translate(-from.x, -from.y, -from.z))
            .compose(&Transform::from(orientation))
    }
}

impl Transformable for Orientation {
    fn transform(self, transform: &Transform) -> Orientation {
        Orientation(self.0.compose(transform))
    }
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: Angle,
    transform: Transform,
    half_height: f64,
    half_width: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(
        hsize: usize,
        vsize: usize,
        mut fov: Angle,
        Orientation(transform): Orientation,
    ) -> Camera {
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

        Camera {
            hsize,
            vsize,
            fov,
            transform,
            half_height,
            half_width,
            pixel_size,
        }
    }

    fn map_ray(&self, pos_x: usize, pos_y: usize) -> Ray {
        let offset_x = (pos_x as f64 + 0.5) * self.pixel_size;
        let offset_y = (pos_y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;
        let inverse_transform = self.transform.invert();
        let pixel = Point::new(world_x, world_y, -1.0).transform(&inverse_transform);
        let origin = Point::new(0.0, 0.0, 0.0).transform(&inverse_transform);
        let direction = (pixel - origin).normalise();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Result<Canvas, WriteError> {
        let mut image = Canvas::new(Width(self.hsize), Height(self.vsize));
        for pos_y in 0..self.vsize {
            for pos_x in 0..self.hsize {
                let ray = self.map_ray(pos_x, pos_y);
                let colour = world.cast_ray(&ray);
                image.paint_colour(pos_x, pos_y, colour)?;
            }
        }
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn view_transform_default() {
        let view_transform = Orientation::view_transform(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::default();
        assert_eq!(view_transform, resulting_transform);
    }

    #[test]
    fn view_transform_pos_z() {
        let view_transform = Orientation::view_transform(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::new(TransformKind::Scale(-1.0, 1.0, -1.0));
        assert_eq!(view_transform, resulting_transform);
    }

    #[test]
    fn view_transform_translate() {
        let view_transform = Orientation::view_transform(
            Point::new(0.0, 0.0, 8.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::new(TransformKind::Translate(0.0, 0.0, -8.0));
        assert_eq!(view_transform, resulting_transform);
    }

    // #[test]
    // fn view_transform_arbitrary() {
    //     let view_transform = Orientation::view_transform(
    //         Point::new(1.0, 3.0, 2.0),
    //         Point::new(4.0, -2.0, 8.0),
    //         Vector::new(1.0, 1.0, 0.0),
    //     );
    //     let resulting_transform = Transform::from(Matrix::from(&vec![
    //         vec![-0.50709, 0.50709, 0.67612, -2.36643],
    //         vec![0.76772, 0.60609, 0.12122, -2.82843],
    //         vec![-0.35857, 0.59761, -0.71714, 0.0],
    //         vec![0.0, 0.0, 0.0, 1.0],
    //     ]));
    //     assert_eq!(view_transform, resulting_transform);
    // }

    #[test]
    fn create_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = Angle::from_radians(FRAC_PI_2);
        let camera = Camera::new(hsize, vsize, fov, Orientation::default());
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.fov, Angle::from_radians(FRAC_PI_2));
        assert_eq!(camera.transform, Transform::default());
    }

    // #[test]
    // fn pixel_size() {
    //     let camera_horizontal_canvas = Camera::new(
    //         200,
    //         125,
    //         Angle::from_radians(std::f64::consts::FRAC_PI_2),
    //         Orientation::default(),
    //     );
    //     let camera_vertical_canvas = Camera::new(
    //         125,
    //         200,
    //         Angle::from_radians(std::f64::consts::FRAC_PI_2),
    //         Orientation::default(),
    //     );
    //     assert_eq!(camera_horizontal_canvas.pixel_size, 0.01);
    //     assert_eq!(camera_vertical_canvas.pixel_size, 0.01);
    // }

    // #[test]
    // fn ray_through_centre_of_camera_view() {
    //     let camera = Camera::new(
    //         201,
    //         101,
    //         Angle::from_radians(FRAC_PI_2),
    //         Orientation::default(),
    //     );
    //     let resulting_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -1.0));
    //     assert_eq!(camera.map_ray(100, 50), resulting_ray);
    // }

    // #[test]
    // fn ray_through_corner_of_camera_view() {
    //     let camera = Camera::new(
    //         201,
    //         101,
    //         Angle::from_radians(FRAC_PI_2),
    //         Orientation::default(),
    //     );
    //     let resulting_ray = Ray::new(
    //         Point::new(0.0, 0.0, 0.0),
    //         Vector::new(0.66519, 0.33259, -0.66851),
    //     );
    //     assert_eq!(camera.map_ray(0, 0), resulting_ray);
    // }

    // #[test]
    // fn ray_with_transformed_camera() {
    //     let transform = Transform::from(vec![
    //         TransformKind::Translate(0.0, -2.0, 5.0),
    //         TransformKind::Rotate(crate::prelude::Axis::Y, Angle::from_radians(FRAC_PI_4)),
    //     ]);
    //     let camera = Camera::new(
    //         201,
    //         101,
    //         Angle::from_radians(FRAC_PI_2),
    //         Orientation::default().transform(&transform),
    //     );
    //     let resulting_ray = Ray::new(
    //         Point::new(0.0, 2.0, -5.0),
    //         Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0),
    //     );
    //     assert_eq!(camera.map_ray(100, 50), resulting_ray);
    // }

    use crate::collections::Colour;
    use crate::scenes::Pixel;
    use crate::utils::Preset;

    #[test]
    fn render_world() {
        let world = World::preset();
        let camera = Camera::new(
            11,
            11,
            Angle::from_radians(FRAC_PI_2),
            Orientation::new(
                Point::new(0.0, 0.0, -5.0),
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            ),
        );
        let image = camera.render(&world).unwrap();
        assert_eq!(
            image[[5, 5]],
            Pixel::new(Colour::new(0.38066, 0.47583, 0.2855))
        );
    }
}
