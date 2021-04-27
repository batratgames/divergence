use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Index };
use crate::maths::{ Vector3D, Matrix2x2 };
use super::InverseMatrixError;

#[derive(Copy, Clone)]
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

    pub fn minor(&self, pos: (usize, usize)) -> Matrix2x2 {
        let mut data: [f32; 4] = [ 0.0, 0.0, 0.0, 0.0 ];

        let mut idx = 0;

        for i in 0..3 {
            if i == pos.0 {
                continue;
            }

            for j in 0..3 {
                if j == pos.1 {
                    continue;
                }

                data[idx] = self[(i, j)];
                idx += 1;
            }
        }

        Matrix2x2::from(data)
    }

    pub fn cofactor(&self) -> Matrix3x3 {
        let mut data: [f32; 9] = [ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ];

        for i in 0..3 {
            for j in 0..3 {
                data[i * 3 + j] = (-1_f32).powf((i+j) as f32) * self.minor((i, j)).determinant();
            }
        }

        Matrix3x3::from(data)
    }

    pub fn adjugate(&self) -> Matrix3x3 {
        self.cofactor().transpose()
    }

    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self.minor((0, 0)).determinant() -
        self[(0, 1)] * self.minor((0, 1)).determinant() +
        self[(0, 2)] * self.minor((0, 2)).determinant()
    }

    pub fn inverse(&self) -> Result<Matrix3x3, InverseMatrixError> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return Err(InverseMatrixError("Matrix has no inverse"));
        }

        Ok((1.0 / determinant) * self.adjugate())
    }

    pub fn col(&self, i: usize) -> Vector3D {
        Vector3D::from((self[(0, i)], self[(1, i)], self[(2, i)]))
    }

    pub fn row(&self, i: usize) -> Vector3D {
        Vector3D::from((self[(i, 0)], self[(i, 1)], self[(i, 2)]))
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        
        for i in 0..3 {
            for j in 0..3 {
                data[i * 3 + j] = self[(i, j)] + rhs[(i, j)];
            }
        }
        
        Matrix3x3::from(data)
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        
        for i in 0..3 {
            for j in 0..3 {
                data[i * 3 + j] = self[(i, j)] - rhs[(i, j)];
            }
        }
        
        Matrix3x3::from(data)
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3::from((self * rhs.col(0), self * rhs.col(1), self * rhs.col(2)))
    }
}

impl Mul<Matrix3x3> for i32 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let mut data = [
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        
        for i in 0..3 {
            for j in 0..3 {
                data[i * 3 + j] = rhs[(i, j)] * (self as f32);
            }
        }
        
        Matrix3x3::from(data)
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
        let mut data = [
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        
        for i in 0..3 {
            for j in 0..3 {
                data[i * 3 + j] = rhs[(i, j)] * self;
            }
        }
        
        Matrix3x3::from(data)
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
        rhs.x() * self.col(0) + rhs.y() * self.col(1) + rhs.z() * self.col(2)
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, rhs: &Self) -> bool {
        for i in 0..9 {
            if self.data[i] != rhs.data[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for Matrix3x3 {}

impl From<[i32; 9]> for Matrix3x3 {
    fn from(data: [i32; 9]) -> Matrix3x3 {
        Matrix3x3 {
            data: [
                data[0] as f32, data[1] as f32, data[2] as f32,
                data[3] as f32, data[4] as f32, data[5] as f32,
                data[6] as f32, data[7] as f32, data[8] as f32,
            ],
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
            data: [ input.0.x(), input.1.x(), input.2.x(), input.0.y(), input.1.y(), input.2.y(), input.0.z(), input.1.z(), input.2.z() ],
        }
    }
}

impl Index<(usize, usize)> for Matrix3x3 {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.1 + 3 * idx.0]
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
    fn minor() {
        let test = Matrix3x3::from(
            [
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
            ]
        ).minor((1, 1));

        let correct = Matrix2x2::from(
            [
                1, 3,
                7, 9,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn cofactor() {
        let test = Matrix3x3::from(
            [
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
            ]
        ).cofactor();

        let correct = Matrix3x3::from(
            [
                -3,   6, -3,
                 6, -12,  6,
                -3,   6, -3,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn adjugate() {
        let test = Matrix3x3::from(
            [
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
            ]
        ).adjugate();

        let correct = Matrix3x3::from(
            [
                -3,   6, -3,
                 6, -12,  6,
                -3,   6, -3,
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
    fn inverse() {
        let test = Matrix3x3::from(
            [
                3, 2, 3, 
                3, 1, 1,
                2, 2, 3,
            ]
        ).inverse().unwrap();

        let correct = Matrix3x3::from(
            [
                 1,  0, -1,
                -7,  3,  6,
                 4, -2, -3,
            ]
        );

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
        let correct = Vector3D::from((14, 32, 50));

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
    fn index() {
        let test = Matrix3x3::from([1, 2, 3, 4, 5, 6, 7, 8, 9])[(2, 1)];
        let correct = 8.0;

        assert_eq!(test, correct);
    }
}
