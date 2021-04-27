use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul };

pub struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).powf(1.0 / 2.0)
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

impl Mul<Vector2D> for f32 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector2D> for i32 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: (self as f32) * rhs.x,
            y: (self as f32) * rhs.y,
        }
    }
}

impl Mul<i32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self
    }
}

impl PartialEq for Vector2D {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Eq for Vector2D {}

impl From<(i32, i32)> for Vector2D {
    fn from(point: (i32, i32)) -> Vector2D {
        Vector2D {
            x: point.0 as f32,
            y: point.1 as f32,
        }
    }
}

impl From<(f32, f32)> for Vector2D {
    fn from(point: (f32, f32)) -> Vector2D {
        Vector2D {
            x: point.0,
            y: point.1,
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
        let test = Vector2D::from((3, 4)).norm();
        let correct = 5.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn sum() {
        let test = Vector2D::from((3,4)).sum();
        let correct = 7.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn x() {
        let test = Vector2D::from((5.0, 4.0)).x();
        let correct = 5.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn y() {
        let test = Vector2D::from((5.0, 4.0)).y();
        let correct = 4.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn xy() {
        let test = Vector2D::from((5.0, 4.0)).xy();
        let correct = (5.0, 4.0);

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Vector2D::from((5.0, 4.0)) + Vector2D::from((3, 2));
        let correct = Vector2D::from((8, 6));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Vector2D::from((5.0, 4.0)) - Vector2D::from((3, 2));
        let correct = Vector2D::from((2, 2));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Vector2D::from((5.0, 4.0)) * Vector2D::from((3, 2));
        let correct = 23.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_left() {
        let test = 5 * Vector2D::from((5.0, 4.0));
        let correct = Vector2D::from((25, 20));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_right() {
        let test = Vector2D::from((5.0, 4.0)) * 5;
        let correct = Vector2D::from((25, 20));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_left() {
        let test = 5.0 * Vector2D::from((5.0, 4.0));
        let correct = Vector2D::from((25, 20));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_right() {
        let test = Vector2D::from((5.0, 4.0)) * 5.0;
        let correct = Vector2D::from((25, 20));

        assert_eq!(test, correct);
    }

    #[test]
    fn from_float() {
        let test = Vector2D::from((5.0, 4.0));
        let correct = Vector2D { x: 5.0, y: 4.0 };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Vector2D::from((5, 4));
        let correct = Vector2D { x: 5.0, y: 4.0 };

        assert_eq!(test, correct);
    }
}
