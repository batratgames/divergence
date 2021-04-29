use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div };

#[derive(Copy, Clone)]
pub struct Vector4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4D {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).powf(1. / 2.)
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z + self.w
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn xy(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn xz(&self) -> (f32, f32) {
        (self.x, self.z)
    }

    pub fn xw(&self) -> (f32, f32) {
        (self.x, self.w)
    }

    pub fn yz(&self) -> (f32, f32) {
        (self.y, self.z)
    }

    pub fn yw(&self) -> (f32, f32) {
        (self.y, self.w)
    }

    pub fn zw(&self) -> (f32, f32) {
        (self.z, self.w)
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn xyw(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.w)
    }

    pub fn xzw(&self) -> (f32, f32, f32) {
        (self.x, self.z, self.w)
    }

    pub fn yzw(&self) -> (f32, f32, f32) {
        (self.y, self.z, self.w)
    }

    pub fn xyzw(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

impl Add for Vector4D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector4D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul for Vector4D {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl<T> Mul<T> for Vector4D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector4D::from((self.x * Into::<f32>::into(rhs), self.y * Into::<f32>::into(rhs), self.z * Into::<f32>::into(rhs), self.w * Into::<f32>::into(rhs)))
    }
}

impl<T> Div<T> for Vector4D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector4D::from((self.x / Into::<f32>::into(rhs), self.y / Into::<f32>::into(rhs), self.z / Into::<f32>::into(rhs), self.w / Into::<f32>::into(rhs)))
    }
}

impl PartialEq for Vector4D {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Eq for Vector4D {}

impl<T> From<(T, T, T, T)> for Vector4D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(point: (T, T, T, T)) -> Vector4D {
        Vector4D {
            x: Into::<f32>::into(point.0),
            y: Into::<f32>::into(point.1),
            z: Into::<f32>::into(point.2),
            w: Into::<f32>::into(point.3),
        }
    }
}

impl fmt::Debug for Vector4D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector4D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm() {
        let test = Vector4D::from((5., 4., 3., 2.)).norm();
        let correct = 54_f32.sqrt();

        assert_eq!(test, correct);
    }

    #[test]
    fn sum() {
        let test = Vector4D::from((1., 2., 3., 4.)).sum();
        let correct = 10.;

        assert_eq!(test, correct);
    }

    #[test]
    fn x() {
        let test = Vector4D::from((5., 4., 3., 2.)).x();
        let correct = 5.;

        assert_eq!(test, correct);
    }

    #[test]
    fn y() {
        let test = Vector4D::from((5., 4., 3., 2.)).y();
        let correct = 4.;

        assert_eq!(test, correct);
    }

    #[test]
    fn z() {
        let test = Vector4D::from((5., 4., 3., 2.)).z();
        let correct = 3.;

        assert_eq!(test, correct);
    }

    #[test]
    fn w() {
        let test = Vector4D::from((5., 4., 3., 2.)).w();
        let correct = 2.;

        assert_eq!(test, correct);
    }

    #[test]
    fn xy() {
        let test = Vector4D::from((5., 4., 3., 2.)).xy();
        let correct = (5., 4.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xz() {
        let test = Vector4D::from((5., 4., 3., 2.)).xz();
        let correct = (5., 3.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xw() {
        let test = Vector4D::from((5., 4., 3., 2.)).xw();
        let correct = (5., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn yz() {
        let test = Vector4D::from((5., 4., 3., 2.)).yz();
        let correct = (4., 3.);

        assert_eq!(test, correct);
    }

    #[test]
    fn yw() {
        let test = Vector4D::from((5., 4., 3., 2.)).yw();
        let correct = (4., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn zw() {
        let test = Vector4D::from((5., 4., 3., 2.)).zw();
        let correct = (3., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyz() {
        let test = Vector4D::from((5., 4., 3., 2.)).xyz();
        let correct = (5., 4., 3.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyw() {
        let test = Vector4D::from((5., 4., 3., 2.)).xyw();
        let correct = (5., 4., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xzw() {
        let test = Vector4D::from((5., 4., 3., 2.)).xzw();
        let correct = (5., 3., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn yzw() {
        let test = Vector4D::from((5., 4., 3., 2.)).yzw();
        let correct = (4., 3., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyzw() {
        let test = Vector4D::from((5., 4., 3., 2.)).xyzw();
        let correct = (5., 4., 3., 2.);

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Vector4D::from((5., 4., 3., 2.)) + Vector4D::from((3., 2., 2., 1.));
        let correct = Vector4D::from((8., 6., 5., 3.));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Vector4D::from((5., 4., 3., 2.)) - Vector4D::from((3., 2., 2., 1.));
        let correct = Vector4D::from((2., 2., 1., 1.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Vector4D::from((5., 4., 3., 2.)) * Vector4D::from((3., 2., 2., 1.));
        let correct = 31.;

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float() {
        let test = Vector4D::from((5., 4., 3., 2.)) * 5.;
        let correct = Vector4D::from((25., 20., 15., 10.));

        assert_eq!(test, correct);
    }

    #[test]
    fn div_float() {
        let test = Vector4D::from((25., 20., 15., 10.)) / 5.;
        let correct = Vector4D::from((5., 4., 3., 2.));

        assert_eq!(test, correct);
    }

    #[test]
    fn from() {
        let test = Vector4D::from((5., 4., 3., 2.));
        let correct = Vector4D { x: 5., y: 4., z: 3., w: 2. };

        assert_eq!(test, correct);
    }
}
