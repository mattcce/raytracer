use raytracer::prelude::*;

#[test]
#[ignore]
fn raycast_sphere() {
    let sphere = Sphere::builder()
        .set_material(Material::preset())
        .build_into();
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
    let floor = Sphere::builder()
        .set_frame_transformation(Transform::new(TransformKind::Scale(10.0, 0.01, 10.0)))
        .set_material(Material {
            pattern: Box::new(Solid::new(Colour::new(1.0, 0.9, 0.9))),
            specular: 0.0,
            ..Material::preset()
        })
        .build_into();
    let left_wall = Sphere::builder()
        .set_frame_transformation(Transform::from(vec![
            TransformKind::Scale(10.0, 0.01, 10.0),
            TransformKind::Rotate(Axis::X, Angle::from_radians(std::f64::consts::FRAC_PI_2)),
            TransformKind::Rotate(Axis::Y, Angle::from_radians(-std::f64::consts::FRAC_PI_4)),
            TransformKind::Translate(0.0, 0.0, 5.0),
        ]))
        .set_material(Material::preset())
        .build_into();
    let right_wall = Sphere::builder()
        .set_frame_transformation(Transform::from(vec![
            TransformKind::Scale(10.0, 0.01, 10.0),
            TransformKind::Rotate(Axis::X, Angle::from_radians(std::f64::consts::FRAC_PI_2)),
            TransformKind::Rotate(Axis::Y, Angle::from_radians(std::f64::consts::FRAC_PI_4)),
            TransformKind::Translate(0.0, 0.0, 5.0),
        ]))
        .set_material(Material::preset())
        .build_into();
    let middle_sphere = Sphere::builder()
        .set_frame_transformation(Transform::new(TransformKind::Translate(-0.5, 1.0, 0.5)))
        .set_material(Material {
            pattern: Box::new(Solid::new(Colour::new(0.1, 1.0, 0.5))),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::preset()
        })
        .build_into();
    let right_sphere = Sphere::builder()
        .set_frame_transformation(Transform::from(vec![
            TransformKind::Scale(0.5, 0.5, 0.5),
            TransformKind::Translate(1.5, 0.5, -0.5),
        ]))
        .set_material(Material {
            pattern: Box::new(Solid::new(Colour::new(0.1, 1.0, 0.5))),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::preset()
        })
        .build_into();
    let left_sphere = Sphere::builder()
        .set_frame_transformation(Transform::from(vec![
            TransformKind::Scale(0.33, 0.33, 0.33),
            TransformKind::Translate(-1.5, 0.33, -0.75),
        ]))
        .set_material(Material {
            pattern: Box::new(Solid::new(Colour::new(1.0, 0.8, 0.1))),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::preset()
        })
        .build_into();
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
