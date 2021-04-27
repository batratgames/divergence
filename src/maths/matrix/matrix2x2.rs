use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Index };
use crate::maths::{ Vector2D, Complex };
use super::InverseMatrixError;

#[derive(Copy, Clone)]
pub struct Matrix2x2 {
    data: [f32; 4]
}

impl Matrix2x2 {
    pub fn zeros() -> Matrix2x2 {
        Matrix2x2 {
            data: [
                0.0, 0.0,
                0.0, 0.0,
            ],
        }
    }

    pub fn ones() -> Matrix2x2 {
        Matrix2x2 {
            data: [
                1.0, 1.0,
                1.0, 1.0,
            ],
        }
    }

    pub fn diagonal(value: f32) -> Matrix2x2 {
        Matrix2x2 {
            data: [
                value, 0.0,
                0.0, value,
            ]
        }
    }

    pub fn identity() -> Matrix2x2 {
        Matrix2x2::diagonal(1.0)
    }

    pub fn transpose(&self) -> Matrix2x2 {
        Matrix2x2 {
            data: [
                self.data[0], self.data[2],
                self.data[1], self.data[3],
            ]
        }
    }

    pub fn minor(&self, pos: (usize, usize)) -> f32 {
        let mut data = 0.0; 

        for i in 0..2 {
            if i == pos.0 {
                continue;
            }

            for j in 0..2 {
                if j == pos.1 {
                    continue;
                }

                data = self[(i, j)];
            }
        }

        return data;
    }

    pub fn cofactor(&self) -> Matrix2x2 {
        let mut data: [f32; 4] = [ 0.0, 0.0, 0.0, 0.0 ];

        for i in 0..2 {
            for j in 0..2 {
                data[i * 2 + j] = (-1_f32).powf((i+j) as f32) * self.minor((i, j));
            }
        }

        Matrix2x2::from(data)
    }

    pub fn adjugate(&self) -> Matrix2x2 {
        self.cofactor().transpose()
    }

    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self.minor((0, 0)) -
        self[(0, 1)] * self.minor((0, 1))
    }

    pub fn inverse(&self) -> Result<Matrix2x2, InverseMatrixError> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return Err(InverseMatrixError("Matrix has no inverse"));
        }

        Ok((1.0 / determinant) * self.adjugate())
    }

    pub fn col(&self, i: usize) -> Vector2D {
        Vector2D::from((self[(0, i)], self[(1, i)]))
    }

    pub fn row(&self, i: usize) -> Vector2D {
        Vector2D::from((self.data[2 * i], self.data[2 * i + 1]))
    }
}

impl Add for Matrix2x2 {
    type Output = Matrix2x2;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [
            0.0, 0.0,
            0.0, 0.0,
        ];
        
        for i in 0..2 {
            for j in 0..2 {
                data[i * 2 + j] = self[(i, j)] + rhs[(i, j)];
            }
        }
        
        Matrix2x2::from(data)
    }
}

impl Sub for Matrix2x2 {
    type Output = Matrix2x2;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [
            0.0, 0.0,
            0.0, 0.0,
        ];
        
        for i in 0..2 {
            for j in 0..2 {
                data[i * 2 + j] = self[(i, j)] - rhs[(i, j)];
            }
        }
        
        Matrix2x2::from(data)
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix2x2::from((self * rhs.col(0), self * rhs.col(1)))
    }
}

impl Mul<Matrix2x2> for i32 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let mut data = [
            0.0, 0.0,
            0.0, 0.0,
        ];
        
        for i in 0..2 {
            for j in 0..2 {
                data[i * 2 + j] = rhs[(i, j)] * (self as f32);
            }
        }
        
        Matrix2x2::from(data)
    }
}

impl Mul<i32> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Matrix2x2> for f32 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let mut data = [
            0.0, 0.0,
            0.0, 0.0,
        ];
        
        for i in 0..2 {
            for j in 0..2 {
                data[i * 2 + j] = rhs[(i, j)] * self;
            }
        }
        
        Matrix2x2::from(data)
    }
}

impl Mul<f32> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector2D> for Matrix2x2 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        rhs.x() * self.col(0) + rhs.y() * self.col(1)
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, rhs: &Self) -> bool {
        for i in 0..4 {
            if self.data[i] != rhs.data[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for Matrix2x2 {}

impl From<[i32; 4]> for Matrix2x2 {
    fn from(data: [i32; 4]) -> Matrix2x2 {
        Matrix2x2 {
            data: [
                data[0] as f32, data[1] as f32,
                data[2] as f32, data[3] as f32
            ],
        }
    }
}

impl From<[f32; 4]> for Matrix2x2 {
    fn from(data: [f32; 4]) -> Matrix2x2 {
        Matrix2x2 {
            data,
        }
    }
}

impl From<(Vector2D, Vector2D)> for Matrix2x2 {
    fn from(input: (Vector2D, Vector2D)) -> Matrix2x2 {
        Matrix2x2 {
            data: [
                input.0.x(), input.1.x(),
                input.0.y(), input.1.y()
            ],
        }
    }
}

impl From<Complex> for Matrix2x2 {
    fn from(input: Complex) -> Matrix2x2 {
        Matrix2x2 {
            data: [
                input.real(), -input.imaginary(),
                input.imaginary(), input.real()
            ]
        }
    }
}

impl Index<(usize, usize)> for Matrix2x2 {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.1 + 2 * idx.0]
    }
}

impl fmt::Debug for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n[{}, {}]\n[{}, {}]", self.data[0], self.data[1], self.data[2], self.data[3])
    }
}

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n({}, {})\n({}, {})", self.data[0], self.data[1], self.data[2], self.data[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        let test = Matrix2x2::zeros();
        let correct = Matrix2x2 {
            data: [
                0.0, 0.0,
                0.0, 0.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn ones() {
        let test = Matrix2x2::ones();
        let correct = Matrix2x2 {
            data: [
                1.0, 1.0, 
                1.0, 1.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn diagonal() {
        let test = Matrix2x2::diagonal(3.0);
        let correct = Matrix2x2 {
            data: [
                3.0, 0.0,
                0.0, 3.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn identity() {
        let test = Matrix2x2::identity();
        let correct = Matrix2x2 {
            data: [
                1.0, 0.0,
                0.0, 1.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn transpose() {
        let test = Matrix2x2::from(
            [
                1, 2, 
                3, 4,
            ]
        ).transpose();

        let correct = Matrix2x2::from(
            [
                1, 3,
                2, 4,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn minor() {
        let test = Matrix2x2::from(
            [
                1, 2,
                3, 4,
            ]
        ).minor((0, 1));

        let correct = 3.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn cofactor() {
        let test = Matrix2x2::from(
            [
                1, 2,
                3, 4,
            ]
        ).cofactor();

        let correct = Matrix2x2::from(
            [
                 4, -3,
                -2,  1,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn adjugate() {
        let test = Matrix2x2::from(
            [
                1, 2,
                3, 4,
            ]
        ).adjugate();

        let correct = Matrix2x2::from(
            [
                 4, -2,
                -3,  1,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn determinant() {
        let test = Matrix2x2::from(
            [
                3, 2,
                3, 3,
            ]
        ).determinant();

        let correct = 3.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn inverse() {
        let test = Matrix2x2::from(
            [
                3, 2,
                3, 3,
            ]
        ).inverse().unwrap();

        let correct = (1.0 / 3.0) * Matrix2x2::from(
            [
                 3, -2,
                -3,  3,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn col() {
        let matrix = Matrix2x2::from(
            [
                1, 2,
                3, 4,
            ]
        );

        let test1 = matrix.col(0);
        let test2 = matrix.col(1);

        let correct1 = Vector2D::from((1, 3));
        let correct2 = Vector2D::from((2, 4));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
    }

    #[test]
    fn row() {
        let matrix = Matrix2x2::from(
            [
                1, 2,
                3, 4,
            ]
        );

        let test1 = matrix.row(0);
        let test2 = matrix.row(1);

        let correct1 = Vector2D::from((1, 2));
        let correct2 = Vector2D::from((3, 4));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
    }

    #[test]
    fn add() {
        let test = Matrix2x2::from([1, 2, 3, 4]) + Matrix2x2::from([4, 3, 2, 1]);
        let correct = Matrix2x2::from([5, 5, 5, 5]);

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Matrix2x2::from([1, 2, 3, 4]) - Matrix2x2::from([4, 3, 2, 1]);
        let correct = Matrix2x2::from([-3, -1, 1, 3]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Matrix2x2::from([1, 2, 3, 4]) * Matrix2x2::identity();
        let correct = Matrix2x2::from([1, 2, 3, 4]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_left() {
        let test = 5 * Matrix2x2::from([1, 2, 3, 4]);
        let correct = Matrix2x2::from([5, 10, 15, 20]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_integer_right() {
        let test = Matrix2x2::from([1, 2, 3, 4]) * 5;
        let correct = Matrix2x2::from([5, 10, 15, 20]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_left() {
        let test = 5.0 * Matrix2x2::from([1, 2, 3, 4]);
        let correct = Matrix2x2::from([5, 10, 15, 20]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float_right() {
        let test = Matrix2x2::from([1, 2, 3, 4]) * 5.0;
        let correct = Matrix2x2::from([5, 10, 15, 20]);

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_vector() {
        let test = Matrix2x2::from([1, 2, 3, 4]) * Vector2D::from((1, 2));
        let correct = Vector2D::from((5, 11));

        assert_eq!(test, correct);
    }
    
    #[test]
    fn from_float() {
        let test = Matrix2x2::from(
            [
                1.0, 2.0,
                3.0, 4.0,
            ]
        );

        let correct = Matrix2x2 {
            data: [
                1.0, 2.0,
                3.0, 4.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Matrix2x2::from(
            [
                1, 2,
                3, 4,
            ]
        );
        
        let correct = Matrix2x2 {
            data: [
                1.0, 2.0,
                3.0, 4.0,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_vector() {
        let test = Matrix2x2::from((Vector2D::from((3, 4)), Vector2D::from((1, 2))));
        let correct = Matrix2x2::from(
            [
                3, 1,
                4, 2,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn from_complex() {
        let test = Matrix2x2::from(Complex::from((3, 4)));
        let correct = Matrix2x2::from(
            [
                3, -4,
                4,  3,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn index() {
        let test = Matrix2x2::from([1, 2, 3, 4])[(1, 0)];
        let correct = 3.0;

        assert_eq!(test, correct);
    }
}
