use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul };

pub struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3D {
    pub fn new() -> Vector3D {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).powf(1.0 / 2.0)
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

    pub fn xy(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn xz(&self) -> (f32, f32) {
        (self.x, self.z)
    }

    pub fn yz(&self) -> (f32, f32) {
        (self.y, self.z)
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vector3D {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vector3D> for f32 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f32> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector3D> for i32 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: (self as f32) * rhs.x,
            y: (self as f32) * rhs.y,
            z: (self as f32) * rhs.z,
        }
    }
}

impl Mul<i32> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Eq for Vector3D {}

impl From<(i32, i32, i32)> for Vector3D {
    fn from(point: (i32, i32, i32)) -> Vector3D {
        Vector3D {
            x: point.0 as f32,
            y: point.1 as f32,
            z: point.2 as f32,
        }
    }
}

impl From<(f32, f32, f32)> for Vector3D {
    fn from(point: (f32, f32, f32)) -> Vector3D {
        Vector3D {
            x: point.0,
            y: point.1,
            z: point.2,
        }
    }
}

impl fmt::Debug for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let test = Vector3D::new();
        let correct = Vector3D { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_float() {
        let test = Vector3D::from((5.0, 4.0, 3.0));
        let correct = Vector3D { x: 5.0, y: 4.0, z: 3.0 };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Vector3D::from((5, 4, 3));
        let correct = Vector3D { x: 5.0, y: 4.0, z: 3.0 };

        assert_eq!(test, correct);
    }

    #[test]
    fn norm() {
        let test = Vector3D::from((5, 4, 3)).norm();
        let correct = 50_f32.sqrt();

        assert_eq!(test, correct);
    }

    #[test]
    fn x() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).x();
        let correct = 5.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn y() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).y();
        let correct = 4.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn z() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).z();
        let correct = 3.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn xy() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).xy();
        let correct = (5.0, 4.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xz() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).xz();
        let correct = (5.0, 3.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn yz() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).yz();
        let correct = (4.0, 3.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyz() {
        let test = Vector3D::from((5.0, 4.0, 3.0)).xyz();
        let correct = (5.0, 4.0, 3.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Vector3D::from((5.0, 4.0, 3.0)) + Vector3D::from((3, 2, 2));
        let correct = Vector3D::from((8, 6, 5));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Vector3D::from((5.0, 4.0, 3.0)) - Vector3D::from((3, 2, 2));
        let correct = Vector3D::from((2, 2, 1));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Vector3D::from((5.0, 4.0, 3.0)) * Vector3D::from((3, 2, 2));
        let correct = 29.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_left() {
        let test = 5 * Vector3D::from((5.0, 4.0, 3.0));
        let correct = Vector3D::from((25, 20, 15));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_right() {
        let test = Vector3D::from((5.0, 4.0, 3.0)) * 5;
        let correct = Vector3D::from((25, 20, 15));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_left() {
        let test = 5.0 * Vector3D::from((5.0, 4.0, 3.0));
        let correct = Vector3D::from((25, 20, 15));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_right() {
        let test = Vector3D::from((5.0, 4.0, 3.0)) * 5.0;
        let correct = Vector3D::from((25, 20, 15));

        assert_eq!(test, correct);
    }
}
