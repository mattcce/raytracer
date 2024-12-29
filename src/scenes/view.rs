use crate::collections::{Angle, Matrix, Point, Vector};
use crate::objects::*;
use crate::scenes::{Canvas, Height, Width, World, WriteError};

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
                let colour = world.cast_ray(ray);
                image.paint_colour(pos_x, pos_y, colour)?;
            }
        }
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use crate::collections::Colour;
    use crate::scenes::Pixel;
    use crate::utils::{approx_eq, BuildInto, Buildable};

    use super::*;

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

    #[test]
    fn view_transform_arbitrary() {
        let view_transform = Orientation::view_transform(
            Point::new(1.0, 3.0, 2.0),
            Point::new(4.0, -2.0, 8.0),
            Vector::new(1.0, 1.0, 0.0),
        );
        let resulting_transform = Transform::from(Matrix::from(&vec![
            vec![-0.507092, 0.507093, 0.676123, -2.366432],
            vec![0.767716, 0.606092, 0.121218, -2.828427],
            vec![-0.358568, 0.597614, -0.717137, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]));
        for i_row in 0..4 {
            for i_col in 0..4 {
                approx_eq!(
                    view_transform.0[[i_row, i_col]],
                    resulting_transform.0[[i_row, i_col]]
                );
            }
        }
    }

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

    #[test]
    fn pixel_size() {
        let camera_horizontal_canvas = Camera::new(
            200,
            125,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
        );
        let camera_vertical_canvas = Camera::new(
            125,
            200,
            Angle::from_radians(std::f64::consts::FRAC_PI_2),
            Orientation::default(),
        );
        approx_eq!(camera_horizontal_canvas.pixel_size, 0.01);
        approx_eq!(camera_vertical_canvas.pixel_size, 0.01);
    }

    #[test]
    fn ray_through_centre_of_camera_view() {
        let camera = Camera::new(
            201,
            101,
            Angle::from_radians(FRAC_PI_2),
            Orientation::default(),
        );
        let specific_ray = camera.map_ray(100, 50);
        let resulting_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -1.0));
        approx_eq!(specific_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(specific_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(specific_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(specific_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(specific_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(specific_ray.direction.z, resulting_ray.direction.z);
    }

    #[test]
    fn ray_through_corner_of_camera_view() {
        let camera = Camera::new(
            201,
            101,
            Angle::from_radians(FRAC_PI_2),
            Orientation::default(),
        );
        let specific_ray = camera.map_ray(0, 0);
        let resulting_ray = Ray::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.665186, 0.332593, -0.668512),
        );
        approx_eq!(specific_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(specific_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(specific_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(specific_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(specific_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(specific_ray.direction.z, resulting_ray.direction.z);
    }

    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn ray_with_transformed_camera() {
        let transform = Transform::from(vec![
            TransformKind::Translate(0.0, -2.0, 5.0),
            TransformKind::Rotate(crate::prelude::Axis::Y, Angle::from_radians(FRAC_PI_4)),
        ]);
        let camera = Camera::new(
            201,
            101,
            Angle::from_radians(FRAC_PI_2),
            Orientation::default().transform(&transform),
        );
        let specific_ray = camera.map_ray(100, 50);
        let resulting_ray = Ray::new(
            Point::new(0.0, 2.0, -5.0),
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0),
        );
        approx_eq!(specific_ray.origin.x, resulting_ray.origin.x);
        approx_eq!(specific_ray.origin.y, resulting_ray.origin.y);
        approx_eq!(specific_ray.origin.z, resulting_ray.origin.z);
        approx_eq!(specific_ray.direction.x, resulting_ray.direction.x);
        approx_eq!(specific_ray.direction.y, resulting_ray.direction.y);
        approx_eq!(specific_ray.direction.z, resulting_ray.direction.z);
    }

    #[test]
    fn render_world() {
        let s1 = Sphere::builder()
            .set_material(Material {
                pattern: Box::new(Solid::new(Colour::new(0.8, 1.0, 0.6))),
                diffuse: 0.7,
                specular: 0.2,
                ..Material::preset()
            })
            .build_into();
        let s2 = Sphere::builder()
            .set_frame_transformation(Transform::new(TransformKind::Scale(0.5, 0.5, 0.5)))
            .set_material(Material::preset())
            .build_into();
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        let world = World {
            objects: vec![s1, s2],
            lights: vec![light],
        };
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
