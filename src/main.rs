extern crate raytracer;

use raytracer::collections::*;
use std::thread;
use std::time;

struct Scene {
    gravity: Vector,
    wind: Vector,
    projectile: Projectile,
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

fn main() {
    let projectile1 = Projectile {
        position: Point::new(0.0, 100.0, 0.0),
        velocity: Vector::new(10.0, 0.0, 0.0),
    };
    let mut scene1 = Scene {
        gravity: Vector::new(0.0, -0.981, 0.0),
        wind: Vector::new(-0.1, 0.0, 0.0),
        projectile: projectile1,
    };

    loop {
        scene1.tick();
        println!(
            "Projectile position: {:?}, velocity: {:?}.",
            scene1.projectile.position, scene1.projectile.velocity
        );
        thread::sleep(time::Duration::from_millis(100));
    }
}

impl Scene {
    fn tick(&mut self) {
        let mut projectile = &mut self.projectile;
        projectile.position = projectile.position + projectile.velocity;
        projectile.velocity = projectile.velocity + self.gravity + self.wind;
    }
}
