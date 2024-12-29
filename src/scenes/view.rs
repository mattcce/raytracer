use crate::collections::{Matrix, Point, Vector};
use crate::objects::*;
use crate::scenes::{
    Canvas, Height, RayGenerator, TaggedPixel, TaggedRay, Width, World, WriteError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Orientation(pub Transform);

impl Orientation {
    pub fn new(from: Point, to: Point, up: Vector) -> Orientation {
        Orientation(Orientation::view_transform(from, to, up))
    }

    pub fn frame_transformation(&self) -> &Transform {
        &self.0
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
pub struct Camera<R: RayGenerator> {
    ray_generator: R,
}

impl<R: RayGenerator> Camera<R> {
    pub fn new(ray_generator: R) -> Camera<R> {
        Camera {
            ray_generator: ray_generator,
        }
    }

    pub fn render(self, world: &World) -> Result<Canvas, WriteError> {
        let (hsize, vsize) = self.ray_generator.canvas_size();
        let mut image = Canvas::new(Width(hsize), Height(vsize));
        for tagged_ray in self.ray_generator {
            let cast_ray = tagged_ray.ray();
            let colour = world.cast_ray(cast_ray);
            let tagged_pixels = tagged_ray.pixels();
            for tagged_pixel in tagged_pixels {
                let [pos_x, pos_y] = tagged_pixel.index();
                image.paint_colour(pos_x, pos_y, colour)?;
            }
        }
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use crate::collections::*;
    use crate::scenes::*;
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
        let native_ray_generator = Native::new(
            11,
            11,
            Angle::from_radians(FRAC_PI_2),
            Orientation::new(
                Point::new(0.0, 0.0, -5.0),
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            ),
        );
        let camera = Camera::new(native_ray_generator);
        let image = camera.render(&world).unwrap();
        assert_eq!(
            image[[5, 5]],
            Pixel::new(Colour::new(0.38066, 0.47583, 0.2855))
        );
    }
}
