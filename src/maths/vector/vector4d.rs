use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul };

pub struct Vector4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4D {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).powf(1.0 / 2.0)
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

impl Mul<Vector4D> for f32 {
    type Output = Vector4D;

    fn mul(self, rhs: Vector4D) -> Self::Output {
        Vector4D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Mul<f32> for Vector4D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector4D> for i32 {
    type Output = Vector4D;

    fn mul(self, rhs: Vector4D) -> Self::Output {
        Vector4D {
            x: (self as f32) * rhs.x,
            y: (self as f32) * rhs.y,
            z: (self as f32) * rhs.z,
            w: (self as f32) * rhs.w,
        }
    }
}

impl Mul<i32> for Vector4D {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self
    }
}

impl PartialEq for Vector4D {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Eq for Vector4D {}

impl From<(i32, i32, i32, i32)> for Vector4D {
    fn from(point: (i32, i32, i32, i32)) -> Vector4D {
        Vector4D {
            x: point.0 as f32,
            y: point.1 as f32,
            z: point.2 as f32,
            w: point.3 as f32,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Vector4D {
    fn from(point: (f32, f32, f32, f32)) -> Vector4D {
        Vector4D {
            x: point.0,
            y: point.1,
            z: point.2,
            w: point.3,
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
        let test = Vector4D::from((5, 4, 3, 2)).norm();
        let correct = 54_f32.sqrt();

        assert_eq!(test, correct);
    }

    #[test]
    fn sum() {
        let test = Vector4D::from((1, 2, 3, 4)).sum();
        let correct = 10.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn x() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).x();
        let correct = 5.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn y() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).y();
        let correct = 4.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn z() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).z();
        let correct = 3.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn w() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).w();
        let correct = 2.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn xy() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xy();
        let correct = (5.0, 4.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xz() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xz();
        let correct = (5.0, 3.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xw();
        let correct = (5.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn yz() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).yz();
        let correct = (4.0, 3.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn yw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).yw();
        let correct = (4.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn zw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).zw();
        let correct = (3.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyz() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xyz();
        let correct = (5.0, 4.0, 3.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xyw();
        let correct = (5.0, 4.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xzw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xzw();
        let correct = (5.0, 3.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn yzw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).yzw();
        let correct = (4.0, 3.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyzw() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)).xyzw();
        let correct = (5.0, 4.0, 3.0, 2.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)) + Vector4D::from((3, 2, 2, 1));
        let correct = Vector4D::from((8, 6, 5, 3));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)) - Vector4D::from((3, 2, 2, 1));
        let correct = Vector4D::from((2, 2, 1, 1));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)) * Vector4D::from((3, 2, 2, 1));
        let correct = 31.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_left() {
        let test = 5 * Vector4D::from((5.0, 4.0, 3.0, 2.0));
        let correct = Vector4D::from((25, 20, 15, 10));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_right() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)) * 5;
        let correct = Vector4D::from((25, 20, 15, 10));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_left() {
        let test = 5.0 * Vector4D::from((5.0, 4.0, 3.0, 2.0));
        let correct = Vector4D::from((25, 20, 15, 10));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_right() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0)) * 5.0;
        let correct = Vector4D::from((25, 20, 15, 10));

        assert_eq!(test, correct);
    }

    #[test]
    fn from_float() {
        let test = Vector4D::from((5.0, 4.0, 3.0, 2.0));
        let correct = Vector4D { x: 5.0, y: 4.0, z: 3.0, w: 2.0 };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Vector4D::from((5, 4, 3, 2));
        let correct = Vector4D { x: 5.0, y: 4.0, z: 3.0, w: 2.0 };

        assert_eq!(test, correct);
    }
}
