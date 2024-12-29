use raytracer::prelude::*;

#[test]
#[ignore]
fn raycast_sphere() {
    let sphere = Sphere::preset();
    let light = Light::new(Point::new(10.0, 10.0, 10.0), Colour::new(1.0, 1.0, 1.0));
    let world = World::new(vec![sphere], vec![light]);
    let camera = Camera::new(
        100,
        100,
        Angle::from_radians(std::f64::consts::FRAC_PI_2),
        Orientation::new(
            Point::new(10.0, 10.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        ),
    );
    camera
        .render(&world)
        .unwrap()
        .output_to_ppm("test_raycast_sphere.ppm")
        .unwrap();
}

#[test]
#[ignore]
fn raycast_scene() {
    let floor = Sphere::new(
        Transform::new(TransformKind::Scale(10.0, 0.01, 10.0)),
        Material {
            colour: Colour::new(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Material::preset()
        },
    );
    let left_wall = Sphere::new(
        Transform::from(vec![
            TransformKind::Scale(10.0, 0.01, 10.0),
            TransformKind::Rotate(Axis::X, Angle::from_radians(std::f64::consts::FRAC_PI_2)),
            TransformKind::Rotate(Axis::Y, Angle::from_radians(-std::f64::consts::FRAC_PI_4)),
            TransformKind::Translate(0.0, 0.0, 5.0),
        ]),
        Material::preset(),
    );
    let right_wall = Sphere::new(
        Transform::from(vec![
            TransformKind::Scale(10.0, 0.01, 10.0),
            TransformKind::Rotate(Axis::X, Angle::from_radians(std::f64::consts::FRAC_PI_2)),
            TransformKind::Rotate(Axis::Y, Angle::from_radians(std::f64::consts::FRAC_PI_4)),
            TransformKind::Translate(0.0, 0.0, 5.0),
        ]),
        Material::preset(),
    );
    let middle_sphere = Sphere::new(
        Transform::new(TransformKind::Translate(-0.5, 1.0, 0.5)),
        Material {
            colour: Colour::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::preset()
        },
    );
    let right_sphere = Sphere::new(
        Transform::from(vec![
            TransformKind::Scale(0.5, 0.5, 0.5),
            TransformKind::Translate(1.5, 0.5, -0.5),
        ]),
        Material {
            colour: Colour::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::preset()
        },
    );
    let left_sphere = Sphere::new(
        Transform::from(vec![
            TransformKind::Scale(0.33, 0.33, 0.33),
            TransformKind::Translate(-1.5, 0.33, -0.75),
        ]),
        Material {
            colour: Colour::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::preset()
        },
    );
    let light_source = Light::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
    let world = World::new(
        vec![
            floor,
            left_wall,
            right_wall,
            middle_sphere,
            right_sphere,
            left_sphere,
        ],
        vec![light_source],
    );
    let camera = Camera::new(
        100,
        50,
        Angle::from_radians(std::f64::consts::FRAC_PI_3),
        Orientation::new(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );
    let image = camera.render(&world).unwrap();
    image
        .output_to_ppm("test_output_raycast_scene.ppm")
        .unwrap();
}
