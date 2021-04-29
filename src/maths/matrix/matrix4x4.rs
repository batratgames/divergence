use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul, Div, Index };
use crate::maths::{ Vector4D, Matrix3x3, Quaternion };
use super::InverseMatrixError;

#[derive(Copy, Clone)]
pub struct Matrix4x4 {
    data: [f32; 16]
}

impl Matrix4x4 {
    pub fn zeros() -> Matrix4x4 {
        Matrix4x4 {
            data: [
                0., 0., 0., 0.,
                0., 0., 0., 0.,
                0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
        }
    }

    pub fn ones() -> Matrix4x4 {
        Matrix4x4 {
            data: [
                1., 1., 1., 1.,
                1., 1., 1., 1.,
                1., 1., 1., 1.,
                1., 1., 1., 1.,
            ],
        }
    }

    pub fn diagonal(value: f32) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                value, 0., 0., 0.,
                0., value, 0., 0.,
                0., 0., value, 0.,
                0., 0., 0., value,
            ]
        }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::diagonal(1.)
    }

    pub fn transpose(&self) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                self.data[0], self.data[4], self.data[8], self.data[12],
                self.data[1], self.data[5], self.data[9], self.data[13],
                self.data[2], self.data[6], self.data[10], self.data[14],
                self.data[3], self.data[7], self.data[11], self.data[15],
            ]
        }
    }

    pub fn minor(&self, pos: (usize, usize)) -> Matrix3x3 {
        let mut data: [f32; 9] = [
            0., 0., 0.,
            0., 0., 0.,
            0., 0., 0.,
        ];

        let mut idx = 0;

        for i in 0..4 {
            if i == pos.0 {
                continue;
            }

            for j in 0..4 {
                if j == pos.1 {
                    continue;
                }

                data[idx] = self[(i, j)];
                idx += 1;
            }
        }

        Matrix3x3::from(data)
    }

    pub fn cofactor(&self) -> Matrix4x4 {
        let mut data: [f32; 16] = [
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ];

        for i in 0..4 {
            for j in 0..4 {
                data[i * 4 + j] = (-1_f32).powf((i+j) as f32) * self.minor((i, j)).determinant();
            }
        }

        Matrix4x4::from(data)
    }

    pub fn adjugate(&self) -> Matrix4x4 {
        self.cofactor().transpose()
    }

    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self.minor((0, 0)).determinant() -
        self[(0, 1)] * self.minor((0, 1)).determinant() +
        self[(0, 2)] * self.minor((0, 2)).determinant() -
        self[(0, 3)] * self.minor((0, 3)).determinant()
    }

    pub fn inverse(&self) -> Result<Matrix4x4, InverseMatrixError> {
        let determinant = self.determinant();

        if determinant == 0. {
            return Err(InverseMatrixError("Matrix has no inverse"));
        }

        Ok(self.adjugate() / determinant)
    }

    pub fn col(&self, i: usize) -> Vector4D {
        Vector4D::from((self[(0, i)], self[(1, i)], self[(2, i)], self[(3, i)]))
    }

    pub fn row(&self, i: usize) -> Vector4D {
        Vector4D::from((self[(i, 0)], self[(i, 1)], self[(i, 2)], self[(i, 3)]))
    }
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ];
        
        for i in 0..4 {
            for j in 0..4 {
                data[i * 4 + j] = self[(i, j)] + rhs[(i, j)];
            }
        }
        
        Matrix4x4::from(data)
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ];
        
        for i in 0..4 {
            for j in 0..4 {
                data[i * 4 + j] = self[(i, j)] - rhs[(i, j)];
            }
        }
        
        Matrix4x4::from(data)
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4x4::from((self * rhs.col(0), self * rhs.col(1), self * rhs.col(2), self * rhs.col(3)))
    }
}

impl<T> Mul<T> for Matrix4x4 where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Matrix4x4;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = [
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ];
        
        for i in 0..4 {
            for j in 0..4 {
                data[i * 4 + j] = self[(i, j)] * Into::<f32>::into(rhs);
            }
        }
        
        Matrix4x4::from(data)
    }
}

impl Mul<Vector4D> for Matrix4x4 {
    type Output = Vector4D;

    fn mul(self, rhs: Vector4D) -> Self::Output {
        self.col(0) * rhs.x() + self.col(1) * rhs.y() + self.col(2) * rhs.z() + self.col(3) * rhs.w()
    }
}

impl<T> Div<T> for Matrix4x4 where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    type Output = Matrix4x4;

    fn div(self, rhs: T) -> Self::Output {
        let mut data = [
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ];
        
        for i in 0..4 {
            for j in 0..4 {
                data[i * 4 + j] = self[(i, j)] / Into::<f32>::into(rhs);
            }
        }
        
        Matrix4x4::from(data)
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, rhs: &Self) -> bool {
        for i in 0..9 {
            if self.data[i] != rhs.data[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for Matrix4x4 {}

impl<T> From<[T; 16]> for Matrix4x4 where
    f32: std::convert::From<T>,
    T: std::convert::From<f32> + Copy {
    fn from(data: [T; 16]) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                Into::<f32>::into(data[ 0]), Into::<f32>::into(data[ 1]), Into::<f32>::into(data[ 2]), Into::<f32>::into(data[ 3]),
                Into::<f32>::into(data[ 4]), Into::<f32>::into(data[ 5]), Into::<f32>::into(data[ 6]), Into::<f32>::into(data[ 7]),
                Into::<f32>::into(data[ 8]), Into::<f32>::into(data[ 9]), Into::<f32>::into(data[10]), Into::<f32>::into(data[11]),
                Into::<f32>::into(data[12]), Into::<f32>::into(data[13]), Into::<f32>::into(data[14]), Into::<f32>::into(data[15]),
            ],
        }
    }
}

impl From<(Vector4D, Vector4D, Vector4D, Vector4D)> for Matrix4x4 {
    fn from(input: (Vector4D, Vector4D, Vector4D, Vector4D)) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                input.0.x(), input.1.x(), input.2.x(), input.3.x(), 
                input.0.y(), input.1.y(), input.2.y(), input.3.y(),
                input.0.z(), input.1.z(), input.2.z(), input.3.z(),
                input.0.w(), input.1.w(), input.2.w(), input.3.w(),
            ],
        }
    }
}

impl From<Quaternion> for Matrix4x4 {
    fn from(input: Quaternion) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                input.r(), -input.i(), -input.j(), -input.k(),
                input.i(),  input.r(), -input.k(),  input.j(),
                input.j(),  input.k(),  input.r(), -input.i(),
                input.k(), -input.j(),  input.i(),  input.r(),
            ]
        }
    }
}

impl Index<(usize, usize)> for Matrix4x4 {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.1 + 4 * idx.0]
    }
}

impl fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7], self.data[8], self.data[9], self.data[10], self.data[11], self.data[12], self.data[13], self.data[14], self.data[15])
    }
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n({}, {}, {}, {})\n({}, {}, {}, {})\n({}, {}, {}, {})\n({}, {}, {}, {})", self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7], self.data[8], self.data[9], self.data[10], self.data[11], self.data[12], self.data[13], self.data[14], self.data[15])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        let test = Matrix4x4::zeros();
        let correct = Matrix4x4 {
            data: [
                0., 0., 0., 0.,
                0., 0., 0., 0.,
                0., 0., 0., 0.,
                0., 0., 0., 0.,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn ones() {
        let test = Matrix4x4::ones();
        let correct = Matrix4x4 {
            data: [
                1., 1., 1., 1.,
                1., 1., 1., 1.,
                1., 1., 1., 1.,
                1., 1., 1., 1.,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn diagonal() {
        let test = Matrix4x4::diagonal(3.);
        let correct = Matrix4x4 {
            data: [
                3., 0., 0., 0.,
                0., 3., 0., 0.,
                0., 0., 3., 0.,
                0., 0., 0., 3.,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn identity() {
        let test = Matrix4x4::identity();
        let correct = Matrix4x4 {
            data: [
                1., 0., 0., 0.,
                0., 1., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn transpose() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ).transpose();

        let correct = Matrix4x4::from(
            [
                1., 5.,  9., 13.,
                2., 6., 10., 14.,
                3., 7., 11., 15.,
                4., 8., 12., 16.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn minor() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ).minor((1, 1));

        let correct = Matrix3x3::from(
            [
                 1.,  3.,  4.,
                 9., 11., 12.,
                13., 15., 16.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn cofactor() {
        let test = Matrix4x4::from(
            [
                1.,  5.,  9., 13.,
               14.,  2.,  6., 10.,
               11., 15.,  3.,  7.,
                8., 12., 16.,  4.,
            ]
        ).cofactor();

        let correct = Matrix4x4::from(
            [
                 1984.,  -192.,  -192., -2624.,
                -2368.,  1984.,  -192.,  -448.,
                 -192., -2368.,  1984.,  -448.,
                 -192.,  -192., -2368.,  1728.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn adjugate() {
        let test = Matrix4x4::from(
            [
                1.,  5.,  9., 13.,
               14.,  2.,  6., 10.,
               11., 15.,  3.,  7.,
                8., 12., 16.,  4.,
            ]
        ).adjugate();

        let correct = Matrix4x4::from(
            [
                 1984., -2368.,  -192.,  -192.,
                 -192.,  1984., -2368.,  -192.,
                 -192.,  -192.,  1984., -2368.,
                -2624.,  -448.,  -448.,  1728.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn determinant() {
        let test = Matrix4x4::from(
            [
                1.,  5.,  9., 13.,
               14.,  2.,  6., 10.,
               11., 15.,  3.,  7.,
                8., 12., 16.,  4.,
            ]
        ).determinant();

        let correct = -34816.;

        assert_eq!(test, correct);
    }

    #[test]
    fn inverse() {
        let test = Matrix4x4::from(
            [
                 1.,  5.,  9., 13.,
                14.,  2.,  6., 10.,
                11., 15.,  3.,  7.,
                 8., 12., 16.,  4.,
            ]
        ).inverse().unwrap();

        let correct = Matrix4x4::from(
            [
                -31.,  37.,   3.,   3.,
                  3., -31.,  37.,   3.,
                  3.,   3., -31.,  37.,
                 41.,   7.,   7., -27.,
            ]
        ) / 544.;

        assert_eq!(test, correct);
    }

    #[test]
    fn col() {
        let matrix = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        );

        let test1 = matrix.col(0);
        let test2 = matrix.col(1);
        let test3 = matrix.col(2);
        let test4 = matrix.col(3);

        let correct1 = Vector4D::from((1., 5.,  9., 13.));
        let correct2 = Vector4D::from((2., 6., 10., 14.));
        let correct3 = Vector4D::from((3., 7., 11., 15.));
        let correct4 = Vector4D::from((4., 8., 12., 16.));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
        assert_eq!(test3, correct3);
        assert_eq!(test4, correct4);
    }

    #[test]
    fn row() {
        let matrix = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        );

        let test1 = matrix.row(0);
        let test2 = matrix.row(1);
        let test3 = matrix.row(2);
        let test4 = matrix.row(3);

        let correct1 = Vector4D::from(( 1.,  2.,  3.,  4.));
        let correct2 = Vector4D::from(( 5.,  6.,  7.,  8.));
        let correct3 = Vector4D::from(( 9., 10., 11., 12.));
        let correct4 = Vector4D::from((13., 14., 15., 16.));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
        assert_eq!(test3, correct3);
        assert_eq!(test4, correct4);
    }

    #[test]
    fn add() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ) + Matrix4x4::from(
            [
                16., 15., 14., 13., 
                12., 11., 10.,  9., 
                 8.,  7.,  6.,  5., 
                 4.,  3.,  2.,  1.,
            ]
        );

        let correct = Matrix4x4::from(
            [
                17., 17., 17., 17., 
                17., 17., 17., 17., 
                17., 17., 17., 17., 
                17., 17., 17., 17.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn sub() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ) - Matrix4x4::from(
            [
                16., 15., 14., 13.,
                12., 11., 10.,  9.,
                 8.,  7.,  6.,  5.,
                 4.,  3.,  2.,  1.,
            ]
        );

        let correct = Matrix4x4::from(
            [
                -15., -13., -11., -9.,
                 -7.,  -5.,  -3., -1.,
                  1.,   3.,   5.,  7.,
                  9.,  11.,  13., 15.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn mul() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ) * Matrix4x4::identity();
        
        let correct = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_float() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ) * 5.;

        let correct = Matrix4x4::from(
            [
                 5., 10., 15., 20.,
                25., 30., 35., 40.,
                45., 50., 55., 60.,
                65., 70., 75., 80.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn mul_vector() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        ) * Vector4D::from((1., 2., 3., 4.));
        
        let correct = Vector4D::from((30., 70., 110., 150.));

        assert_eq!(test, correct);
    }

    #[test]
    fn div_float() {
        let test = Matrix4x4::from(
            [
                 5., 10., 15., 20.,
                25., 30., 35., 40.,
                45., 50., 55., 60.,
                65., 70., 75., 80.,
            ]
        ) / 5.;

        let correct = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        );

        assert_eq!(test, correct);
    }
    
    #[test]
    fn from() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        );
        
        let correct = Matrix4x4 {
            data: [
                 1.,  2.,  3.,  4.,
                 5.,  6.,  7.,  8.,
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_vector() {
        let test = Matrix4x4::from((Vector4D::from((3., 4., 5., 6.)), Vector4D::from((1., 2., 3., 4.)), Vector4D::from((4., 5., 6., 7.)), Vector4D::from((5., 4., 3., 2.))));
        let correct = Matrix4x4::from(
            [
                3., 1., 4., 5.,
                4., 2., 5., 4.,
                5., 3., 6., 3.,
                6., 4., 7., 2.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn from_quaternion() {
        let test = Matrix4x4::from(Quaternion::from((1., 2., 3., 4.)));
        let correct = Matrix4x4::from(
            [
                1., -2., -3., -4.,
                2.,  1., -4.,  3.,
                3.,  4.,  1., -2.,
                4., -3.,  2.,  1.,
            ]
        );

        assert_eq!(test, correct);
    }

    #[test]
    fn index() {
        let test = Matrix4x4::from(
            [
                 1.,  2.,  3.,  4., 
                 5.,  6.,  7.,  8., 
                 9., 10., 11., 12.,
                13., 14., 15., 16.,
            ]
        )[(2, 1)];
        let correct = 10.;

        assert_eq!(test, correct);
    }
}
