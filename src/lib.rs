fn flt_approx_eq(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < 1.0E-6
}

#[derive(Debug)]
pub struct Tuple {
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

impl Default for Tuple {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
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
pub enum TupleConversionError {
    BadWValue,
}

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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
        assert_eq!(
            Tuple::from(&Point::new(4.0, -4.0, 3.0)),
            Tuple::new(4.0, -4.0, 3.0, 1.0)
        );
    }

    #[test]
    fn create_a_vector() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(Tuple::from(&v), Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_tuples() {
        let t1 = Tuple::new(1.4, -9.16, 25.36, -49.64);
        let t2 = Tuple::new(-1.4, 9.16, -25.36, 49.64);
        let res = Tuple::default();

        assert_eq!(t1 + t2, res);
        assert_eq!(t2 + t1, res);
    }

    #[test]
    fn add_vectors() {
        let v1 = Vector::new(7.2, -1000.0, 0.0);
        let v2 = Vector::new(0.0004, 100.001, -12.5);
        let res = Vector::new(7.2004, -899.999, -12.5);

        assert_eq!(v1 + v2, res);
        assert_eq!(v2 + v1, res);
    }

    #[test]
    fn add_vectors_to_points() {
        let p = Point::new(9.0, 3.1415, 0.01);
        let v = Vector::new(1.0, -1.0, 14.3);
        let res = Point::new(10.0, 2.1415, 14.31);

        assert_eq!(p + v, res);
        assert_eq!(v + p, res);
    }

    #[test]
    fn subtract_tuples() {
        let t1 = Tuple::new(1.4, -9.16, 25.36, -49.64);
        let t2 = Tuple::new(-1.4, 9.16, -25.36, 49.64);

        assert_eq!(t1 - t2, Tuple::new(2.8, -18.32, 50.72, -99.28));
        assert_eq!(t2 - t1, Tuple::new(-2.8, 18.32, -50.72, 99.28));
    }

    #[test]
    fn subtract_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
        assert_eq!(p2 - p1, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        assert_eq!(
            Point::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0),
            Point::new(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
        assert_eq!(v2 - v1, Vector::new(2.0, 4.0, 6.0));
    }
}

#[cfg(doctest)]
mod doctests {
    /// ```compile_fail,E0369
    /// Point::new(12.0, -24.1, 48.2) + Point::new(-12.0, 24.1, -48.2);
    /// ```
    fn _add_points() {}

    /// ```compile_fail,E0369
    /// Vector::new(5.0, 6.0, 7.0) - Point::new(3.0, 2.0, 1.0);
    /// ```
    fn _subtract_point_from_vector() {}
}
