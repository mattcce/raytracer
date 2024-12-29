use raytracer::canvas::*;
use raytracer::collections::*;
use raytracer::objects::*;
use raytracer::ray::*;
use raytracer::transform::{Transform, TransformKind};

#[test]
#[ignore]
fn raycast_sphere() {
    // draw a sphere of radius 5.0 at (10.0, 10.0, 0.0)
    let sphere = Sphere {
        transform: Transform::from(vec![
            TransformKind::Scale(5.0, 5.0, 5.0),
            TransformKind::Translate(10.0, 10.0, 0.0),
        ]),
        ..Sphere::default()
    };

    // raycast source at (10.0, 10.0, -25.0)
    let source = Point::new(10.0, 10.0, -25.0);

    // 100x100 pixel canvas at origin (0.0, 0.0, 25.0) parallel to xy-plane
    let mut canvas = Canvas::new(Width(25), Height(25));

    // pixel to draw
    let colour_red = Colour::new(1.0, 0.0, 0.0);

    // draw each pixel by casting a ray and checking for intersects
    for col in 0..25 {
        for row in 0..25 {
            let vector = Vector::new(col as f64 - 10.0, row as f64 - 10.0, 50.0);
            let ray = Ray::new(source, vector);
            match ray.intersect(&sphere) {
                Some(x) => {
                    println!(
                        "Intersect at t = {}, drawing at {}, {}",
                        x.hit().t(),
                        col,
                        row
                    );
                    canvas.paint_colour(col, row, colour_red).unwrap();
                }
                None => continue,
            }
        }
    }

    // output canvas
    canvas
        .output_to_ppm("test_output_raycast_sphere.ppm")
        .unwrap();
}
