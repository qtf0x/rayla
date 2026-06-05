use std::ops::{Add, Div, Mul, Neg, Sub};

fn flt_approx_eq(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < 1.0E-6
}

#[derive(Clone, Copy, Debug, Default)]
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

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        flt_approx_eq(self.x, other.x)
            && flt_approx_eq(self.y, other.y)
            && flt_approx_eq(self.z, other.z)
            && flt_approx_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl From<Point> for Tuple {
    fn from(p: Point) -> Self {
        Self::new(p.x, p.y, p.z, 1.0)
    }
}

impl From<Vector> for Tuple {
    fn from(p: Vector) -> Self {
        Self::new(p.x, p.y, p.z, 0.0)
    }
}

#[derive(Debug)]
pub enum TupleConversionError {
    BadWValue,
}

#[derive(Clone, Copy, Debug, Default)]
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        flt_approx_eq(self.x, other.x)
            && flt_approx_eq(self.y, other.y)
            && flt_approx_eq(self.z, other.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl TryFrom<Tuple> for Point {
    type Error = TupleConversionError;

    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
        if flt_approx_eq(t.w, 1.0) {
            Ok(Self::new(t.x, t.y, t.z))
        } else {
            Err(Self::Error::BadWValue)
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
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

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        flt_approx_eq(self.x, other.x)
            && flt_approx_eq(self.y, other.y)
            && flt_approx_eq(self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = TupleConversionError;

    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
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

        std::assert_matches!(Point::try_from(a), Ok(_));
        std::assert_matches!(Vector::try_from(a), Err(_));
    }

    #[test]
    fn tuple_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        std::assert_matches!(Point::try_from(a), Err(_));
        std::assert_matches!(Vector::try_from(a), Ok(_));
    }

    #[test]
    fn create_a_point() {
        assert_eq!(
            Tuple::from(Point::new(4.0, -4.0, 3.0)),
            Tuple::new(4.0, -4.0, 3.0, 1.0)
        );
    }

    #[test]
    fn create_a_vector() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(Tuple::from(v), Tuple::new(4.0, -4.0, 3.0, 0.0));
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

    #[test]
    fn negate_tuple() {
        assert_eq!(
            -Tuple::new(1.0, -2.0, 3.0, -4.0),
            Tuple::new(-1.0, 2.0, -3.0, 4.0)
        );
    }

    #[test]
    fn negate_vector() {
        assert_eq!(-Vector::new(1.0, -2.0, 3.0), Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scale_multiply_tuples() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let res_bigger = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let res_smaller = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(t * 3.5, res_bigger);
        assert_eq!(3.5 * t, res_bigger);
        assert_eq!(t * 0.5, res_smaller);
        assert_eq!(0.5 * t, res_smaller);
    }

    #[test]
    fn scale_multiply_vectors() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let res_bigger = Vector::new(3.5, -7.0, 10.5);
        let res_smaller = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(v * 3.5, res_bigger);
        assert_eq!(3.5 * v, res_bigger);
        assert_eq!(v * 0.5, res_smaller);
        assert_eq!(0.5 * v, res_smaller);
    }

    #[test]
    fn scale_divide_tuples() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) / 2.0,
            Tuple::new(0.5, -1.0, 1.5, -2.0)
        );
    }

    #[test]
    fn scale_divide_vectors() {
        assert_eq!(
            Vector::new(1.0, -2.0, 3.0) / 2.0,
            Vector::new(0.5, -1.0, 1.5)
        );
    }

    #[test]
    fn vector_magnitudes() {
        assert!(flt_approx_eq(Vector::new(1.0, 0.0, 0.0).len(), 1.0));
        assert!(flt_approx_eq(Vector::new(0.0, 1.0, 0.0).len(), 1.0));
        assert!(flt_approx_eq(Vector::new(0.0, 0.0, 1.0).len(), 1.0));
        assert!(flt_approx_eq(Vector::new(1.0, 2.0, 3.0).len(), 14.0.sqrt()));
        assert!(flt_approx_eq(
            Vector::new(-1.0, -2.0, -3.0).len(),
            14.0.sqrt()
        ));
    }
}

#[cfg(doctest)]
mod doctests {
    /// ```compile_fail,E0369
    /// use rayla::*;
    ///
    /// Point::new(12.0, -24.1, 48.2) + Point::new(-12.0, 24.1, -48.2);
    /// ```
    fn _add_points() {}

    /// ```compile_fail,E0369
    /// use rayla::*;
    ///
    /// Vector::new(5.0, 6.0, 7.0) - Point::new(3.0, 2.0, 1.0);
    /// ```
    fn _subtract_point_from_vector() {}

    /// ```compile_fail,E0600
    /// use rayla::*;
    ///
    /// -Point::new(1.0, -2.0, 3.0);
    /// ```
    fn _negate_point() {}

    /// ```compile_fail,E0369
    /// use rayla::*;
    ///
    /// Point::new(1.0, 2.0, 3.0) * 3.5;
    /// ```
    fn _multiply_point_by_scalar() {}

    /// ```compile_fail,E0369
    /// use rayla::*;
    ///
    /// 3.5 * Point::new(1.0, 2.0, 3.0);
    /// ```
    fn _multiply_scalar_by_point() {}

    /// ```compile_fail,E0369
    /// use rayla::*;
    ///
    /// Point::new(1.0, 2.0, 3.0) / 2.0;
    /// ```
    fn _scale_divide_points() {}
}
