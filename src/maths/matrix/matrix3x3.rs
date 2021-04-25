use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Index };
use crate::maths::Vector3D;

pub struct Matrix3x3 {
    data: [f32; 9]
}

impl Matrix3x3 {
    pub fn zeros() -> Matrix3x3 {
        Matrix3x3 {
            data: [
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ],
        }
    }

    pub fn ones() -> Matrix3x3 {
        Matrix3x3 {
            data: [
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
            ],
        }
    }

    pub fn diagonal(value: f32) -> Matrix3x3 {
        Matrix3x3 {
            data: [
                value, 0.0, 0.0,
                0.0, value, 0.0,
                0.0, 0.0, value,
            ]
        }
    }

    pub fn identity() -> Matrix3x3 {
        Matrix3x3::diagonal(1.0)
    }

    pub fn transpose(&self) -> Matrix3x3 {
        Matrix3x3 {
            data: [
                self.data[0], self.data[3], self.data[6],
                self.data[1], self.data[4], self.data[7],
                self.data[2], self.data[5], self.data[8],
            ]
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }

    pub fn col(&self, i: usize) -> Vector3D {
        Vector3D::from((self.data[i], self.data[i + 2], self.data[i + 6]))
    }

    pub fn row(&self, i: usize) -> Vector3D {
        Vector3D::from((self.data[3 * i], self.data[3 * i + 1], self.data[3 * i + 2]))
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            data: [
                self.data[0] + rhs.data[0], self.data[1] + rhs.data[1], 0.0,
                self.data[2] + rhs.data[2], self.data[3] + rhs.data[3], 0.0,
                0.0, 0.0, 0.0,
            ]
        }
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            data: [
                self.data[0] - rhs.data[0], self.data[1] - rhs.data[1], 0.0,
                self.data[2] - rhs.data[2], self.data[3] - rhs.data[3], 0.0,
                0.0, 0.0, 0.0,
            ]
        }
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3 { 
            data: [
                self.data[0] * rhs.data[0] + self.data[1] * rhs.data[2], self.data[0] * rhs.data[1] + self.data[1] * rhs.data[3], 0.0,
                self.data[2] * rhs.data[0] + self.data[3] * rhs.data[2], self.data[2] * rhs.data[1] + self.data[3] * rhs.data[3], 0.0,
                0.0, 0.0, 0.0,
            ]
        }
    }
}

impl Mul<Matrix3x3> for i32 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        Matrix3x3 {
            data: [
                (self as f32) * rhs.data[0], (self as f32) * rhs.data[1], 0.0, 
                (self as f32) * rhs.data[2], (self as f32) * rhs.data[3], 0.0,
                0.0, 0.0, 0.0,
            ]
        }
    }
}

impl Mul<i32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Matrix3x3> for f32 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        Matrix3x3 {
            data: [
                self * rhs.data[0], self * rhs.data[1], 0.0,
                self * rhs.data[2], self * rhs.data[3], 0.0,
                0.0, 0.0, 0.0,
            ]
        }
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector3D> for Matrix3x3 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        rhs.x() * self.col(0) + rhs.y() * self.col(1)
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.data[0] == rhs.data[0] &&
        self.data[1] == rhs.data[1] &&
        self.data[2] == rhs.data[2] &&
        self.data[3] == rhs.data[3]
    }
}

impl Eq for Matrix3x3 {}

impl From<[i32; 9]> for Matrix3x3 {
    fn from(data: [i32; 9]) -> Matrix3x3 {
        Matrix3x3 {
            data: [data[0] as f32, data[1] as f32, data[2] as f32, data[3] as f32, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl From<[f32; 9]> for Matrix3x3 {
    fn from(data: [f32; 9]) -> Matrix3x3 {
        Matrix3x3 {
            data,
        }
    }
}

impl From<(Vector3D, Vector3D, Vector3D)> for Matrix3x3 {
    fn from(input: (Vector3D, Vector3D, Vector3D)) -> Matrix3x3 {
        Matrix3x3 {
            data: [ input.0.x(), input.1.x(), input.0.y(), input.1.y(), 0.0, 0.0, 0.0, 0.0, 0.0 ],
        }
    }
}

impl Index<(usize, usize)> for Matrix3x3 {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.1 + 2 * idx.0]
    }
}

impl fmt::Debug for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n[{}, {}, {}]\n[{}, {}, {}]\n[{}, {}, {}]", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7], self.data[8])
    }
}

impl fmt::Display for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n({}, {}, {})\n({}, {}, {})\n({}, {}, {})", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7], self.data[8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        let test = Matrix3x3::zeros();
        let correct = Matrix3x3 {
            data: [
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn ones() {
        let test = Matrix3x3::ones();
        let correct = Matrix3x3 {
            data: [
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn diagonal() {
        let test = Matrix3x3::diagonal(3.0);
        let correct = Matrix3x3 {
            data: [
                3.0, 0.0, 0.0,
                0.0, 3.0, 0.0,
                0.0, 0.0, 3.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn identity() {
        let test = Matrix3x3::identity();
        let correct = Matrix3x3 {
            data: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn transpose() {
        let test = Matrix3x3::from(
            [
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
            ]
        ).transpose();

        let correct = Matrix3x3::from(
            [
                1, 4, 7,
                2, 5, 8, 
                3, 6, 9,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn from_float() {
        let test = Matrix3x3::from(
            [
                1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0,
            ]
        );
        
        let correct = Matrix3x3 {
            data: [
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Matrix3x3::from(
            [
                1, 2, 3, 
                4, 5, 6,
                7, 8, 9,
            ]
        );

        let correct = Matrix3x3 {
            data: [
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_vector() {
        let test = Matrix3x3::from((Vector3D::from((3, 4, 5)), Vector3D::from((1, 2, 3)), Vector3D::from((4, 5, 6))));
        let correct = Matrix3x3::from(
            [
                3, 1, 4,
                4, 2, 5,
                5, 3, 6,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn determinant() {
        let test = Matrix3x3::from(
            [
                3, 2, 3, 
                3, 1, 1,
                2, 2, 3,
            ]
        ).determinant();

        let correct = 1.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn col() {
        let matrix = Matrix3x3::from(
            [
                1, 2, 3, 
                4, 5, 6,
                7, 8, 9,
            ]
        );

        let test1 = matrix.col(0);
        let test2 = matrix.col(1);
        let test3 = matrix.col(2);

        let correct1 = Vector3D::from((1, 4, 7));
        let correct2 = Vector3D::from((2, 5, 8));
        let correct3 = Vector3D::from((3, 6, 9));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
        assert_eq!(test3, correct3);
    }

    #[test]
    fn row() {
        let matrix = Matrix3x3::from(
            [
                1, 2, 3, 
                4, 5, 6,
                7, 8, 9,
            ]
        );

        let test1 = matrix.row(0);
        let test2 = matrix.row(1);
        let test3 = matrix.row(2);

        let correct1 = Vector3D::from((1, 2, 3));
        let correct2 = Vector3D::from((4, 5, 6));
        let correct3 = Vector3D::from((7, 8, 9));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
        assert_eq!(test3, correct3);
    }

    #[test]
    fn add() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]) + Matrix3x3::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let correct = Matrix3x3::from([10, 10, 10, 10, 10, 10, 10, 10, 10]);

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]) - Matrix3x3::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let correct = Matrix3x3::from([-8, -6, -4, -2, 0, 2, 4, 6, 8]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]) * Matrix3x3::identity();
        let correct = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_left() {
        let test = 5 * Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let correct = Matrix3x3::from([5, 10, 15, 20, 25, 30, 35, 40, 45]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_right() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]) * 5;
        let correct = Matrix3x3::from([5, 10, 15, 20, 25, 30, 35, 40, 45]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_left() {
        let test = 5.0 * Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let correct = Matrix3x3::from([5, 10, 15, 20, 25, 30, 35, 40, 45]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_right() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]) * 5.0;
        let correct = Matrix3x3::from([5, 10, 15, 20, 25, 30, 35, 40, 45]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_vector() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9]) * Vector3D::from((1, 2, 3));
        let correct = Vector3D::from((14, 32, 40));

        assert_eq!(test, correct);
    }

    #[test]
    fn index() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9])[(2, 1)];
        let correct = 7.0;

        assert_eq!(test, correct);
    }
}
