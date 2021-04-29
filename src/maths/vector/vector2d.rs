use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div };

#[derive(Copy, Clone)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).powf(1. / 2.)
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn xy(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector2D {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> Mul<T> for Vector2D where 
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2D::from((self.x * Into::<f32>::into(rhs), self.y * Into::<f32>::into(rhs)))
    }
}

impl<T> Div<T> for Vector2D where 
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector2D::from((self.x / Into::<f32>::into(rhs), self.y / Into::<f32>::into(rhs)))
    }
}

impl PartialEq for Vector2D {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Eq for Vector2D {}

impl<T> From<(T, T)> for Vector2D where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(point: (T, T)) -> Vector2D {
        Vector2D {
            x: Into::<f32>::into(point.0),
            y: Into::<f32>::into(point.1),
        }
    }
}

impl fmt::Debug for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm() {
        let test = Vector2D::from((3., 4.)).norm();
        let correct = 5.;

        assert_eq!(test, correct);
    }

    #[test]
    fn sum() {
        let test = Vector2D::from((3., 4.)).sum();
        let correct = 7.;

        assert_eq!(test, correct);
    }

    #[test]
    fn x() {
        let test = Vector2D::from((5., 4.)).x();
        let correct = 5.;

        assert_eq!(test, correct);
    }

    #[test]
    fn y() {
        let test = Vector2D::from((5., 4.)).y();
        let correct = 4.;

        assert_eq!(test, correct);
    }

    #[test]
    fn xy() {
        let test = Vector2D::from((5., 4.)).xy();
        let correct = (5., 4.);

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Vector2D::from((5., 4.)) + Vector2D::from((3., 2.));
        let correct = Vector2D::from((8., 6.));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Vector2D::from((5., 4.)) - Vector2D::from((3., 2.));
        let correct = Vector2D::from((2., 2.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Vector2D::from((5., 4.)) * Vector2D::from((3., 2.));
        let correct = 23.;

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float() {
        let test = Vector2D::from((5., 4.)) * 5.;
        let correct = Vector2D::from((25., 20.));

        assert_eq!(test, correct);
    }

    #[test]
    fn div_float() {
        let test = Vector2D::from((25., 20.)) / 5.;
        let correct = Vector2D::from((5., 4.));

        assert_eq!(test, correct);
    }

    #[test]
    fn from() {
        let test = Vector2D::from((5., 4.));
        let correct = Vector2D { x: 5., y: 4. };

        assert_eq!(test, correct);
    }
}
