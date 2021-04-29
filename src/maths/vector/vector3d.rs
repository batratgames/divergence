use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div, Rem };

#[derive(Copy, Clone)]
pub struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3D {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).powf(1. / 2.)
    }

    pub fn cross(&self, rhs: Vector3D) -> Vector3D {
        Vector3D::from(
            (
                self.y() * rhs.z() - self.z() * rhs.y(),
                self.z() * rhs.x() - self.x() * rhs.z(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            )
        )
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
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

impl<T> Mul<T> for Vector3D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3D::from((self.x * Into::<f32>::into(rhs), self.y * Into::<f32>::into(rhs), self.z * Into::<f32>::into(rhs)))
    }
}

impl<T> Div<T> for Vector3D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector3D::from((self.x / Into::<f32>::into(rhs), self.y / Into::<f32>::into(rhs), self.z / Into::<f32>::into(rhs)))
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

impl Rem for Vector3D {
    type Output = Self;

    fn rem(self, rhs: Vector3D) -> Self::Output {
        self.cross(rhs)
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Eq for Vector3D {}

impl<T> From<(T, T, T)> for Vector3D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(point: (T, T, T)) -> Vector3D {
        Vector3D {
            x: Into::<f32>::into(point.0),
            y: Into::<f32>::into(point.1),
            z: Into::<f32>::into(point.2),
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
    fn norm() {
        let test = Vector3D::from((5., 4., 3.)).norm();
        let correct = 50_f32.sqrt();

        assert_eq!(test, correct);
    }

    #[test]
    fn cross() {
        let test = Vector3D::from((1., 2., 3.)).cross(Vector3D::from((3., 2., 1.)));
        let correct = Vector3D::from((-4., 8., -4.));

        assert_eq!(test, correct);
    }

    #[test]
    fn sum() {
        let test = Vector3D::from((1., 2., 3.)).sum();
        let correct = 6.;

        assert_eq!(test, correct);
    }

    #[test]
    fn x() {
        let test = Vector3D::from((5., 4., 3.)).x();
        let correct = 5.;

        assert_eq!(test, correct);
    }

    #[test]
    fn y() {
        let test = Vector3D::from((5., 4., 3.)).y();
        let correct = 4.;

        assert_eq!(test, correct);
    }

    #[test]
    fn z() {
        let test = Vector3D::from((5., 4., 3.)).z();
        let correct = 3.;

        assert_eq!(test, correct);
    }

    #[test]
    fn xy() {
        let test = Vector3D::from((5., 4., 3.)).xy();
        let correct = (5., 4.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xz() {
        let test = Vector3D::from((5., 4., 3.)).xz();
        let correct = (5., 3.);

        assert_eq!(test, correct);
    }

    #[test]
    fn yz() {
        let test = Vector3D::from((5., 4., 3.)).yz();
        let correct = (4., 3.);

        assert_eq!(test, correct);
    }

    #[test]
    fn xyz() {
        let test = Vector3D::from((5., 4., 3.)).xyz();
        let correct = (5., 4., 3.);

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Vector3D::from((5., 4., 3.)) + Vector3D::from((3., 2., 2.));
        let correct = Vector3D::from((8., 6., 5.));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Vector3D::from((5., 4., 3.)) - Vector3D::from((3., 2., 2.));
        let correct = Vector3D::from((2., 2., 1.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Vector3D::from((5., 4., 3.)) * Vector3D::from((3., 2., 2.));
        let correct = 29.;

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float() {
        let test = Vector3D::from((5., 4., 3.)) * 5.;
        let correct = Vector3D::from((25., 20., 15.));

        assert_eq!(test, correct);
    }

    #[test]
    fn div_float() {
        let test = Vector3D::from((25., 20., 15.)) / 5.;
        let correct = Vector3D::from((5., 4., 3.));

        assert_eq!(test, correct);
    }

    #[test]
    fn rem() {
        let test = Vector3D::from((1., 2., 3.)) % Vector3D::from((3., 2., 1.));
        let correct = Vector3D::from((-4., 8., -4.));

        assert_eq!(test, correct);
    }

    #[test]
    fn from() {
        let test = Vector3D::from((5., 4., 3.));
        let correct = Vector3D { x: 5., y: 4., z: 3. };

        assert_eq!(test, correct);
    }
}
