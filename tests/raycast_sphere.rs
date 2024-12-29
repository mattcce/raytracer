use raytracer::prelude::*;

#[test]
#[ignore]
fn raycast_sphere() {
    // 100x100 pixel canvas with origin (0.0, 0.0, 100.0) parallel to xy-plane
    let canvas_width: usize = 100;
    let canvas_height: usize = 100;
    let mut canvas = Canvas::new(canvas::Width(canvas_width), canvas::Height(canvas_height));

    // sphere of radius 10.0 at (50.0, 50.0, 50.0)
    let sphere = Sphere {
        transform: Transform::from(vec![
            TransformKind::Scale(10.0, 10.0, 10.0),
            TransformKind::Translate(50.0, 50.0, 50.0),
        ]),
        material: Material {
            colour: Colour::new(1.0, 0.2, 1.0),
            ..Material::default()
        },
    };

    // white light source at (25.0, 75.0, 25.0)
    let light = PointLight::new(Point::new(25.0, 25.0, 25.0), Colour::new(1.0, 1.0, 1.0));

    // draw pixels by casting a ray and checking for intersects
    for x in 0..100 {
        for y in 0..100 {
            // raycast source at (50.0, 50.0, 0.0)
            let ray = Ray::new(
                Point::new(50.0, 50.0, 0.0),
                Vector::new(x as f64 - 50.0, y as f64 - 50.0, 100.0).normalise(),
            );
            let intersections = ray.intersect(&sphere);
            let hit = match intersections.hit() {
                Some(hit) => hit,
                None => continue,
            };
            let point = ray.position(hit.t());
            let normal = hit.object().normal_at(point);
            let eye = -ray.direction;
            let colour =
                raytracer::objects::light::lighting(sphere.material, light, point, eye, normal);
            canvas.paint_colour(x, y, colour).unwrap();
        }
    }

    // output canvas
    canvas
        .output_to_ppm("test_output_raycast_sphere.ppm")
        .unwrap();
}
