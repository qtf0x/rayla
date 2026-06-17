// SPDX-License-Identifier: AGPL-3.0-or-later

/* This file is part of the Rayla ray tracer.
 * Copyright (C) 2026 Aster Marias <aster@slware.org>
 *
 * Rayla is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rayla is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Rayla.  If not, see <https://www.gnu.org/licenses/>. */

//! Mathematical structures and operations for 3D rendering.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// Floating-point type determining precision of all internal operations.
pub type Real = f64;

/// Types which conceptually respect an
/// [equivalence relation](https://en.wikipedia.org/wiki/Equivalence_relation), but which have an
/// imprecise machine representation.
///
/// Generally, these types are subsets of uncountable sets; for example, floating-point types like
/// [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) and
/// [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) conceptually represent the real
/// numbers ℝ, but are of course really only the subsets of ℝ representable by the respective IEEE
/// 754-2008 types. A literal equality relation comparing the bit representations of these numbers
/// is simple, but often not what we want. Due to the imprecision inherent in representing most (in
/// the limit, *all*) real numbers in this way, calculations that ought to result in the same
/// *exact* value will often instead have different floating-point values, depending on the
/// implementation details and handling of intermediate values. In these cases, we would rather have
/// a way to compare these values for *approximate* equality, within some arbitrary error bound
/// deemed fit for a specific application.
///
/// This trait is modeled after
/// [`std::cmp::PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html). If Rust allowed
/// for arbitrary operator overloading, it might further provide analogous operators like `~~` and
/// `!~`. Like `PartialEq`, the relations represented by `ApproxEq` are *not* necessarily reflexive
/// (thank you, `NaN`).
pub trait ApproxEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn approx_eq(&self, other: &Rhs) -> bool;

    fn approx_ne(&self, other: &Rhs) -> bool {
        !self.approx_eq(other)
    }
}

impl ApproxEq for Real {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < 1.0E-6
    }
}

/// Four packed real number values. Not a four-dimensional vector.
#[derive(Clone, Copy, Debug, Default)]
pub struct Tuple {
    x: Real,
    y: Real,
    z: Real,
    w: Real,
}

impl Tuple {
    fn new(x: Real, y: Real, z: Real, w: Real) -> Self {
        Self { x, y, z, w }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        Real::approx_eq(&self.x, &other.x)
            && Real::approx_eq(&self.y, &other.y)
            && Real::approx_eq(&self.z, &other.z)
            && Real::approx_eq(&self.w, &other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
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

impl Mul<Real> for Tuple {
    type Output = Self;

    fn mul(self, scalar: Real) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<Tuple> for Real {
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

impl Div<Real> for Tuple {
    type Output = Self;

    fn div(self, scalar: Real) -> Self::Output {
        Self::Output {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

/// Euclidean to [projective](https://en.wikipedia.org/wiki/Homogeneous_coordinates) (homogenous)
/// coordinates.
impl From<Point> for Tuple {
    fn from(p: Point) -> Self {
        Self::new(p.x, p.y, p.z, 1.0)
    }
}

/// Euclidean to [projective](https://en.wikipedia.org/wiki/Homogeneous_coordinates) (homogenous)
/// coordinates.
impl From<Vector> for Tuple {
    fn from(v: Vector) -> Self {
        Self::new(v.x, v.y, v.z, 0.0)
    }
}

#[derive(Debug)]
pub enum TupleConversionError {
    BadWValue(Real),
}

/// Element of a three-dimensional, real-valued color (vector) space.
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorRGB {
    pub r: Real,
    pub g: Real,
    pub b: Real,
}

impl ColorRGB {
    pub fn new(r: Real, g: Real, b: Real) -> Self {
        Self { r, g, b }
    }
}

impl PartialEq for ColorRGB {
    fn eq(&self, other: &Self) -> bool {
        Real::approx_eq(&self.r, &other.r)
            && Real::approx_eq(&self.g, &other.g)
            && Real::approx_eq(&self.b, &other.b)
    }
}

impl Add for ColorRGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for ColorRGB {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

/// Performs element-wise
/// ([Hadamard/Shur](https://en.wikipedia.org/wiki/Hadamard_product_(matrices))) product.
impl Mul for ColorRGB {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<ColorRGB> for Real {
    type Output = ColorRGB;

    fn mul(self, rhs: ColorRGB) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Mul<Real> for ColorRGB {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<Real> for ColorRGB {
    type Output = Self;

    fn div(self, rhs: Real) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

/// Position in three-dimensional Euclidean space.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Point {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        Real::approx_eq(&self.x, &other.x)
            && Real::approx_eq(&self.y, &other.y)
            && Real::approx_eq(&self.z, &other.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, start: Self) -> Self::Output {
        Self::Output {
            x: self.x - start.x,
            y: self.y - start.y,
            z: self.z - start.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, translation: Vector) -> Self::Output {
        Self::Output {
            x: self.x + translation.x,
            y: self.y + translation.y,
            z: self.z + translation.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, translation: Vector) -> Self::Output {
        Self::Output {
            x: self.x - translation.x,
            y: self.y - translation.y,
            z: self.z - translation.z,
        }
    }
}

impl TryFrom<Tuple> for Point {
    type Error = TupleConversionError;

    /// Attempt to interpret tuple as a position in projective space.
    ///
    /// ## Errors
    ///
    /// Returns [`TupleConversionError::BadWValue`] if the fourth element of the tuple is not
    /// (approximately) equal to one. Error contains the offending *w* value.
    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
        if Real::approx_eq(&t.w, &1.0) {
            Ok(Self::new(t.x, t.y, t.z))
        } else {
            Err(Self::Error::BadWValue(t.w))
        }
    }
}

/// Three-dimensional Euclidean vector.
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    x: Real,
    y: Real,
    z: Real,
}

impl Vector {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.len();
        let magnitude = if magnitude.approx_ne(&0.0) {
            magnitude
        } else {
            1.0
        };

        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot(&self, rhs: &Self) -> Real {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        Real::approx_eq(&self.x, &other.x)
            && Real::approx_eq(&self.y, &other.y)
            && Real::approx_eq(&self.z, &other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

    fn add(self, position: Point) -> Self::Output {
        Self::Output {
            x: self.x + position.x,
            y: self.y + position.y,
            z: self.z + position.z,
        }
    }
}

impl Mul<Real> for Vector {
    type Output = Self;

    fn mul(self, scalar: Real) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector> for Real {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<Real> for Vector {
    type Output = Self;

    fn div(self, scalar: Real) -> Self::Output {
        Self::Output {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = TupleConversionError;

    /// Attempt to interpret tuple as a vector in projective space.
    ///
    /// ## Errors
    ///
    /// Returns [`TupleConversionError::BadWValue`] if the fourth element of the tuple is not
    /// (approximately) equal to zero. Error contains the offending *w* value.
    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
        if Real::approx_eq(&t.w, &0.0) {
            Ok(Self::new(t.x, t.y, t.z))
        } else {
            Err(Self::Error::BadWValue(t.w))
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
    fn tuple_is_neither() {
        let t1 = Tuple::new(4.3, -4.2, 3.1, 0.001);
        let t2 = Tuple::new(4.3, -4.2, 3.1, 100_000.0);

        std::assert_matches!(Point::try_from(t1), Err(_));
        std::assert_matches!(Vector::try_from(t1), Err(_));
        std::assert_matches!(Point::try_from(t2), Err(_));
        std::assert_matches!(Vector::try_from(t2), Err(_));
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
        assert!(Real::approx_eq(&Vector::new(1.0, 0.0, 0.0).len(), &1.0));
        assert!(Real::approx_eq(&Vector::new(0.0, 1.0, 0.0).len(), &1.0));
        assert!(Real::approx_eq(&Vector::new(0.0, 0.0, 1.0).len(), &1.0));
        assert!(Real::approx_eq(
            &Vector::new(1.0, 2.0, 3.0).len(),
            &(14.0 as Real).sqrt()
        ));
        assert!(Real::approx_eq(
            &Vector::new(-1.0, -2.0, -3.0).len(),
            &(14.0 as Real).sqrt()
        ));
    }

    #[test]
    fn magnitude_of_zero_vector() {
        assert!(Real::approx_eq(&Vector::default().len(), &0.0));
    }

    #[test]
    fn normalize_vectors() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v1.normalize(), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(
            v2.normalize(),
            Vector::new(1.0, 2.0, 3.0) / (14.0 as Real).sqrt()
        );
    }

    #[test]
    fn normalized_vector_is_unit() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert!(Real::approx_eq(&v.normalize().len(), &1.0));
    }

    #[test]
    fn normalize_zero_vector() {
        let zero = Vector::default();

        assert_eq!(zero.normalize(), zero);
    }

    #[test]
    fn vector_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert!(Real::approx_eq(&v1.dot(&v2), &20.0));
        assert!(Real::approx_eq(&v2.dot(&v1), &20.0));
    }

    #[test]
    fn vector_cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
        assert_ne!(v1.cross(&v2), v2.cross(&v1));
    }

    #[test]
    fn color_rgb_access_fields() {
        let c = ColorRGB::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn add_colors_rgb() {
        let c1 = ColorRGB::new(0.9, 0.6, 0.75);
        let c2 = ColorRGB::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, ColorRGB::new(1.6, 0.7, 1.0));
        assert_eq!(c1 + c2, c2 + c1);
    }

    #[test]
    fn subtract_colors_rgb() {
        let c1 = ColorRGB::new(0.9, 0.6, 0.75);
        let c2 = ColorRGB::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, ColorRGB::new(0.2, 0.5, 0.5));
        assert_eq!(c2 - c1, ColorRGB::new(-0.2, -0.5, -0.5));
    }

    #[test]
    fn scale_multiply_colors_rgb() {
        let c = ColorRGB::new(0.2, 0.3, 0.4);

        assert_eq!(2.0 * c, ColorRGB::new(0.4, 0.6, 0.8));
        assert_eq!(c * 2.0, 2.0 * c);

        assert_eq!(-0.5 * c, ColorRGB::new(-0.1, -0.15, -0.2));
        assert_eq!(c * -0.5, -0.5 * c);
    }

    #[test]
    fn scale_divide_colors_rgb() {
        let c = ColorRGB::new(0.2, 0.3, 0.4);

        assert_eq!(c / 2.0, ColorRGB::new(0.1, 0.15, 0.2));
        assert_eq!(c / -0.5, ColorRGB::new(-0.4, -0.6, -0.8));
    }

    #[test]
    fn multiply_colors_rgb() {
        let c1 = ColorRGB::new(1.0, 0.2, 0.4);
        let c2 = ColorRGB::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, ColorRGB::new(0.9, 0.2, 0.04));
        assert_eq!(c1 * c2, c2 * c1);
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
