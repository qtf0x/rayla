use rayla::{Point, Tuple, Vector};

fn main() {
    let v = Vector::new(1.0, -2.0, 3.0);
    let p = Point::new(-3.0, 2.0, -1.0);

    println!("Here's a vector: {:?}", Tuple::from(&v));
    println!("Here's a point:  {:?}", Tuple::from(&p));
}
