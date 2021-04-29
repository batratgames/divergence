use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div };

#[derive(Copy, Clone)]
pub struct Complex {
    real: f32,
    imaginary: f32,
}

impl Complex {
    /// This will return the real component of the `Complex` number
    ///
    /// # Example
    ///
    /// ```
    /// # use divergence::maths::Complex;
    /// #
    /// let z = Complex::from((3., 4.));
    /// let a = z.real();
    /// # assert_eq!(a, 3.);
    /// ```
    ///
    pub fn real(&self) -> f32 {
        self.real
    }

    pub fn imaginary(&self) -> f32 {
        self.imaginary
    }

    pub fn norm(&self) -> f32 {
        (self.real * self.real + self.imaginary * self.imaginary).powf(1. / 2.)
    }

    pub fn argument(&self) -> f32 {
        (self.imaginary / self.real).atan()
    }

    pub fn conjugate(&self) -> Complex {
        Complex::from((self.real, -self.imaginary))
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
        Complex::from((self.real * rhs.real - self.imaginary * rhs.imaginary, self.real * rhs.imaginary + self.imaginary * rhs.real))
    }
}

impl<T> Mul<T> for Complex where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Complex::from((self.real() * Into::<f32>::into(rhs), self.imaginary() * Into::<f32>::into(rhs)))
    }
}

impl Div for Complex {
    type Output = Self;
    
    fn div(self, rhs: Complex) -> Self::Output {
        Complex::from((self.real * rhs.real + self.imaginary * rhs.imaginary, self.imaginary * rhs.real - self.real * rhs.imaginary)) / (rhs.real * rhs.real + rhs.imaginary * rhs.imaginary)
    }
}

impl<T> Div<T> for Complex where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Complex::from((self.real() / Into::<f32>::into(rhs), self.imaginary() / Into::<f32>::into(rhs)))
    }
}

impl PartialEq for Complex {
    fn eq(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.imaginary == rhs.imaginary
    }
}

impl Eq for Complex {}

impl<T> From<(T, T)> for Complex where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(data: (T, T)) -> Complex {
        Complex {
            real: Into::<f32>::into(data.0),
            imaginary: Into::<f32>::into(data.1),
        }
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
        let test = Complex::from((3., 4.)).real();
        let correct = 3.;

        assert_eq!(test, correct);
    }

    #[test]
    fn imaginary() {
        let test = Complex::from((3., 4.)).imaginary();
        let correct = 4.;

        assert_eq!(test, correct);
    }

    #[test]
    fn norm() {
        let test = Complex::from((3., 4.)).norm();
        let correct = 5.;

        assert_eq!(test, correct);
    }

    #[test]
    fn argument() {
        let test = Complex::from((3., 0.)).argument();
        let correct = 0.;

        assert_eq!(test, correct);
    }

    #[test]
    fn conjugate() {
        let test = Complex::from((3., 4.)).conjugate();
        let correct = Complex::from((3., -4.));

        assert_eq!(test, correct);
    }

    #[test]
    fn add() {
        let test = Complex::from((3., 4.)) + Complex::from((1., 1.));
        let correct = Complex::from((4., 5.));

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Complex::from((3., 4.)) - Complex::from((1., 1.));
        let correct = Complex::from((2., 3.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Complex::from((3., 4.)) * Complex::from((1., 1.));
        let correct = Complex::from((-1., 7.));

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float() {
        let test = Complex::from((3., 4.)) * 5.;
        let correct = Complex::from((15., 20.));

        assert_eq!(test, correct);
    }

    #[test]
    fn div() {
        let test = Complex::from((3., 4.)) / Complex::from((1., 1.));
        let correct = Complex::from((3.5, 0.5));

        assert_eq!(test, correct);
    }

    #[test]
    fn div_float() {
        let test = Complex::from((0., 1.)) / 2.;
        let correct = Complex::from((0., 0.5));
    }

    #[test]
    fn from_tuple() {
        let test = Complex::from((3., 4.));
        let correct = Complex {
            real: 3.,
            imaginary: 4.,
        };

        assert_eq!(test, correct);
    }
}
