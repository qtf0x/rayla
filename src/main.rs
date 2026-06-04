fn main() {
    let v = Vector::new(1.0, -2.0, 3.0);
    let p = Point::new(-3.0, 2.0, -1.0);

    println!("Here's a vector: {:?}", Tuple::from(&v));
    println!("Here's a point:  {:?}", Tuple::from(&p));
}

fn flt_approx_eq(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < 1.0E-6
}

#[derive(Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        flt_approx_eq(self.x, other.x)
            && flt_approx_eq(self.y, other.y)
            && flt_approx_eq(self.z, other.z)
            && flt_approx_eq(self.w, other.w)
    }
}

impl From<&Point> for Tuple {
    fn from(p: &Point) -> Self {
        Self::new(p.x, p.y, p.z, 1.0)
    }
}

impl From<&Vector> for Tuple {
    fn from(p: &Vector) -> Self {
        Self::new(p.x, p.y, p.z, 0.0)
    }
}

#[derive(Debug)]
enum TupleConversionError {
    BadWValue,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl TryFrom<&Tuple> for Point {
    type Error = TupleConversionError;

    fn try_from(t: &Tuple) -> Result<Self, Self::Error> {
        if flt_approx_eq(t.w, 1.0) {
            Ok(Self::new(t.x, t.y, t.z))
        } else {
            Err(Self::Error::BadWValue)
        }
    }
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl TryFrom<&Tuple> for Vector {
    type Error = TupleConversionError;

    fn try_from(t: &Tuple) -> Result<Self, Self::Error> {
        if flt_approx_eq(t.w, 0.0) {
            Ok(Self::new(t.x, t.y, t.z))
        } else {
            Err(Self::Error::BadWValue)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        std::assert_matches!(Point::try_from(&a), Ok(_));
        std::assert_matches!(Vector::try_from(&a), Err(_));
    }

    #[test]
    fn tuple_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        std::assert_matches!(Point::try_from(&a), Err(_));
        std::assert_matches!(Vector::try_from(&a), Ok(_));
    }

    #[test]
    fn create_a_point() {
        let p = Point::new(4.0, -4.0, 3.0);

        assert_eq!(Tuple::from(&p), Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn create_a_vector() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(Tuple::from(&v), Tuple::new(4.0, -4.0, 3.0, 0.0));
    }
}
