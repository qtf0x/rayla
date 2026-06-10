use rayla::math::{Point, Vector};

fn main() {
    // projectile starts one unit above the origin
    // velocity is normalized to 1 unit/tick
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    // gravity and wind both in units/tick
    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut ticks: u32 = 0;
    while p.position.y > 0.0 {
        println!("[Tick {ticks:3}] Projectile position: {:?}", p.position);

        p = p.tick(&e);
        ticks += 1;
    }
    println!("[Tick {ticks:3}] Projectile position: {:?}", p.position);

    println!("It took {ticks} time units for the projectile to hit the ground!");
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn tick(&self, env: &Environment) -> Self {
        Self {
            position: self.position + self.velocity,
            velocity: self.velocity + env.gravity + env.wind,
        }
    }
}
