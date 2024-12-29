use raytracer::prelude::*;

struct Scene {
    gravity: Vector,
    wind: Vector,
    projectile: Projectile,
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Scene {
    fn tick(&mut self) {
        let mut projectile = &mut self.projectile;
        projectile.position = projectile.position + projectile.velocity;
        projectile.velocity = projectile.velocity + self.gravity + self.wind;
    }
}

#[test]
#[ignore]
fn basic_2d_projectile_motion_scene() {
    let projectile1 = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalise() * 11.25,
    };
    let mut scene1 = Scene {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
        projectile: projectile1,
    };
    let mut canvas = Canvas::new(canvas::Width(900), canvas::Height(550));

    loop {
        let pos_x = match scene1.projectile.position.x.round() {
            x if x >= 0.0 => x as usize,
            _ => break,
        };
        let pos_y = match scene1.projectile.position.y.round() {
            y if y >= 0.0 => 550 - y as usize,
            _ => break,
        };
        if canvas
            .paint_colour(pos_x, pos_y, Colour::new(1.0, 0.0, 0.0))
            .is_err()
        {
            break;
        } else {
            scene1.tick();
        }
    }

    canvas.output_to_ppm("test_output_projmotion.ppm").unwrap();
}
