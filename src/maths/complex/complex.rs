use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div };

#[derive(Copy, Clone)]
pub struct Complex {
    real: f32,
    imaginary: f32,
}

impl Complex {
    pub fn real(&self) -> f32 {
        self.real
    }

    pub fn imaginary(&self) -> f32 {
        self.imaginary
    }

    pub fn norm(&self) -> f32 {
        (self.real * self.real + self.imaginary * self.imaginary).powf(1.0 / 2.0)
    }

    pub fn argument(&self) -> f32 {
        (self.imaginary / self.real).atan()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::from((self.real + rhs.real, self.imaginary + rhs.imaginary))
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex::from((self.real - rhs.real, self.imaginary - rhs.imaginary))
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::from([self.norm() * rhs.norm(), self.argument() + rhs.argument()])
    }
}

impl Mul<i32> for Complex {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Complex::from((self.real() * (rhs as f32), self.imaginary() * (rhs as f32)))
    }
}

impl Mul<Complex> for i32 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Complex::from((self.real() * rhs, self.imaginary() * rhs))
    }
}

impl Mul<Complex> for f32 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}

impl Div for Complex {
    type Output = Self;
    
    fn div(self, rhs: Complex) -> Self::Output {
        Complex::from([self.norm() / rhs.norm(), self.argument() - rhs.argument()])
    }
}

impl Div<f32> for Complex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Complex::from((self.real() / rhs, self.imaginary() / rhs))
    }
}

impl Div<Complex> for f32 {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        rhs / self
    }
}

impl PartialEq for Complex {
    fn eq(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.imaginary == rhs.imaginary
    }
}

impl Eq for Complex {}

impl From<(i32, i32)> for Complex {
    fn from(data: (i32, i32)) -> Complex {
        Complex::from((data.0 as f32, data.1 as f32))
    }
}

impl From<(f32, f32)> for Complex {
    fn from(data: (f32, f32)) -> Complex {
        Complex {
            real: data.0,
            imaginary: data.1,
        }
    }
}

impl From<[f32; 2]> for Complex {
    fn from(data: [f32; 2]) -> Complex {
        Complex::from((data[0] * data[1].cos(), data[0] * data[1].sin()))
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}i]", self.real, self.imaginary)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}i)", self.real, self.imaginary)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn real() {
        let test = Complex::from((3, 4)).real();
        let correct = 3.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn imaginary() {
        let test = Complex::from((3, 4)).imaginary();
        let correct = 4.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn norm() {
        let test = Complex::from((3, 4)).norm();
        let correct = 5.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn argument() {
        let test = Complex::from((3, 3)).argument();
        let correct = (1.0 / 2_f32.sqrt()).asin() - 0.0000001;

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Complex::from((3, 4)) + Complex::from((1, 1));
        let correct = Complex::from((4, 5));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Complex::from((3, 4)) - Complex::from((1, 1));
        let correct = Complex::from((2, 3));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Complex::from((3, 4)) * Complex::from((1, 1));
        let correct = Complex::from((-1, 7));

        assert_eq!(test, correct);
    }

    #[test]
    fn div() {
        let test = Complex::from((3, 4)) / Complex::from((1, 1));
        let correct = Complex::from((3.5, 0.5));

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Complex::from((3, 4));
        let correct = Complex {
            real: 3.0,
            imaginary: 4.0,
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_float() {
        let test = Complex::from((3.0, 4.0));
        let correct = Complex {
            real: 3.0,
            imaginary: 4.0,
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_list() {
        let test = Complex::from([3.0, std::f32::consts::PI / 2.0]);
        let correct = Complex {
            real: 0.0,
            imaginary: 3.0,
        };

        assert_eq!(test, correct);
    }
}
