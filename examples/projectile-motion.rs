use std::{
    fs::{self, File},
    io,
    path::Path,
};

use rayla::{
    Canvas,
    encode::{ImageEncoder, netpbm::PpmEncoder},
    math::{ColorRGB, Point, Vector},
};

fn main() -> io::Result<()> {
    // projectile starts one unit above the origin
    // velocity is normalized to 1 unit/tick
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    // gravity and wind both in units/tick
    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);
    let orange = ColorRGB::new(1.0, 0.7, 0.0);

    let mut ticks: u32 = 0;
    while p.position.y > 0.0 {
        println!("[Tick {ticks:3}] Projectile position: {:?}", p.position);

        let (x_quantized, y_quantized) = (
            p.position.x.round() as u16,
            canvas.height - p.position.y.round() as u16,
        );
        canvas.write(x_quantized, y_quantized, orange);

        p = p.tick(&e);
        ticks += 1;
    }
    println!("[Tick {ticks:3}] Projectile position: {:?}", p.position);

    println!("It took {ticks} time units for the projectile to hit the ground!");

    let dir = Path::new("images/");
    if !dir.exists() {
        fs::create_dir(dir)?;
    }

    let mut file = File::create(dir.join("projectile.ppm"))?;
    PpmEncoder::raw().encode(&canvas, &mut file)?;

    Ok(())
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
