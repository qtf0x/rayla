use std::f64::consts::TAU;

use rayla::{Matrix4, Point, Vector, rotation_y, translation};

fn main() {
    let v = Vector::new(1.0, -2.0, 3.0);
    let p = Point::new(-3.0, 2.0, -1.0);

    println!("Here's a vector: {:?}", v);
    println!("Here's a point:  {:?}", p);

    let twelve = Point::new(0.0, 0.0, 1.0);
    let to_screen = translation(20.0, 0.0, 20.0);

    println!("\nClock face (looking down the y axis):");
    for hour in 0..12 {
        let rotate = rotation_y(hour as f64 * TAU / 12.0);
        let position = (to_screen * rotate) * twelve;

        println!("  {:>2}h -> {:?}", hour, position);
    }

    let spun = Matrix4::IDENTITY
        .rotate_y(TAU / 4.0)
        .translate(5.0, 0.0, 0.0)
        * Point::new(1.0, 0.0, 0.0);

    println!("\nChained rotate+translate of (1, 0, 0): {:?}", spun);
}
