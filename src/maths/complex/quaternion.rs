use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div };
use crate::maths::Vector3D;

#[derive(Copy, Clone)]
pub struct Quaternion {
    r: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl Quaternion {
    pub fn r(&self) -> f32 {
        self.r
    }
    
    pub fn i(&self) -> f32 {
        self.i
    }
    
    pub fn j(&self) -> f32 {
        self.j
    }
    
    pub fn k(&self) -> f32 {
        self.k
    }

    pub fn norm(&self) -> f32 {
        (self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k).powf(1. / 2.)
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::from((self.r, -self.i, -self.j, -self.k))
    }

    pub fn inverse(&self) -> Quaternion {
        self.conjugate() / self.norm().powf(2.)
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion::from((
            self.r + rhs.r,
            self.i + rhs.i,
            self.j + rhs.j,
            self.k + rhs.k,
        ))
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Quaternion) -> Self::Output {
        Quaternion::from((
            self.r - rhs.r,
            self.i - rhs.i,
            self.j - rhs.j,
            self.k - rhs.k,
        ))
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion::from(
            (
                self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
                self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
                self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i,
                self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r,
            )
        )
    }
}

impl<T> Mul<T> for Quaternion where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Quaternion;

    fn mul(self, rhs: T) -> Self::Output {
        Quaternion::from((self.r * Into::<f32>::into(rhs), self.i * Into::<f32>::into(rhs), self.j * Into::<f32>::into(rhs), self.k * Into::<f32>::into(rhs)))
    }
}

impl Div for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: Quaternion) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<T> Div<T> for Quaternion where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Quaternion;

    fn div(self, rhs: T) -> Self::Output {
        Quaternion::from((self.r / Into::<f32>::into(rhs), self.i / Into::<f32>::into(rhs), self.j / Into::<f32>::into(rhs), self.k / Into::<f32>::into(rhs)))
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, rhs: &Self) -> bool {
        self.r == rhs.r && self.i == rhs.i && self.j == rhs.j && self.k == rhs.k
    }
}

impl Eq for Quaternion {}

impl<T> From<(T, T, T, T)> for Quaternion where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(data: (T, T, T, T)) -> Quaternion {
        Quaternion {
            r: Into::<f32>::into(data.0),
            i: Into::<f32>::into(data.1),
            j: Into::<f32>::into(data.2),
            k: Into::<f32>::into(data.3),
        }
    }
}

impl<T> From<(T, Vector3D)> for Quaternion where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(data: (T, Vector3D)) -> Quaternion {
        Quaternion {
            r: Into::<f32>::into(data.0),
            i: data.1.x(),
            j: data.1.y(),
            k: data.1.z(),
        }
    }
}

impl fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}i, {}j, {}k]", self.r, self.i, self.j, self.k)
    }
}

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}i, {}j, {}k)", self.r, self.i, self.j, self.k)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn r() {
        let test = Quaternion::from((1., 2., 3., 4.)).r();
        let correct = 1.;

        assert_eq!(test, correct);
    }

    #[test]
    fn i() {
        let test = Quaternion::from((1., 2., 3., 4.)).i();
        let correct = 2.;

        assert_eq!(test, correct);
    }

    #[test]
    fn j() {
        let test = Quaternion::from((1., 2., 3., 4.)).j();
        let correct = 3.;

        assert_eq!(test, correct);
    }

    #[test]
    fn k() {
        let test = Quaternion::from((1., 2., 3., 4.)).k();
        let correct = 4.;

        assert_eq!(test, correct);
    }

    #[test]
    fn norm() {
        let test = Quaternion::from((1., 2., 3., 4.)).norm();
        let correct = 30_f32.sqrt();

        assert_eq!(test, correct);
    }

    #[test]
    fn conjugate() {
        let test = Quaternion::from((1., 2., 3., 4.)).conjugate();
        let correct = Quaternion::from((1., -2., -3., -4.));

        assert_eq!(test, correct);
    }

    #[test]
    fn inverse() {
        let test = Quaternion::from((1., 1., 1., 1.)).inverse();
        let correct = Quaternion::from((1., -1., -1., -1.)) / 4.;

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Quaternion::from((1., 2., 3., 4.)) + Quaternion::from((4., 3., 2., 1.));
        let correct = Quaternion::from((5., 5., 5., 5.));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Quaternion::from((1., 2., 3., 4.)) - Quaternion::from((4., 3., 2., 1.));
        let correct = Quaternion::from((-3., -1., 1., 3.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Quaternion::from((1., 2., 3., 4.)) * Quaternion::from((4., 3., 2., 1.));
        let correct = Quaternion::from((-12., 6., 24., 12.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float() {
        let test = Quaternion::from((1., 2., 3., 4.)) * 5.;
        let correct = Quaternion::from((5., 10., 15., 20.));

        assert_eq!(test, correct);
    }

    #[test]
    fn div_float() {
        let test = Quaternion::from((5., 10., 15., 20.)) / 5.;
        let correct = Quaternion::from((1., 2., 3., 4.));

        assert_eq!(test, correct);
    }
}
