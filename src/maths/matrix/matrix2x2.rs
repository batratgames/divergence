use std::fmt;
use std::cmp::{ PartialEq, Eq };
use std::ops::{ Add, Sub, Mul };
use super::super::vector::vector2d::Vector2D;

pub struct Matrix2x2 {
    data: [f32; 4]
}

impl Matrix2x2 {
    pub fn zeros() -> Matrix2x2 {
        Matrix2x2 {
            data: [ 0.0, 0.0, 0.0, 0.0 ],
        }
    }

    pub fn ones() -> Matrix2x2 {
        Matrix2x2 {
            data: [ 1.0, 1.0, 1.0, 1.0 ],
        }
    }

    pub fn identity() -> Matrix2x2 {
        Matrix2x2 {
            data: [ 1.0, 0.0, 0.0, 1.0 ],
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
}

impl From<[i32; 4]> for Matrix2x2 {
    fn from(data: [i32; 4]) -> Matrix2x2 {
        Matrix2x2 {
            data: [data[0] as f32, data[1] as f32, data[2] as f32, data[3] as f32],
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

impl From<(i32, i32, i32, i32)> for Matrix2x2 {
    fn from(input: (i32, i32, i32, i32)) -> Matrix2x2 {
        Matrix2x2 {
            data: [input.0 as f32, input.1 as f32, input.2 as f32, input.3 as f32],
        }
    }
}

impl From<(f32, f32, f32, f32)> for Matrix2x2 {
    fn from(input: (f32, f32, f32, f32)) -> Matrix2x2 {
        Matrix2x2 {
            data: [input.0, input.1, input.2, input.3],
        }
    }
}

impl From<(Vector2D, Vector2D)> for Matrix2x2 {
    fn from(input: (Vector2D, Vector2D)) -> Matrix2x2 {
        Matrix2x2 {
            data: [ input.0.x(), input.1.x(), input.0.y(), input.1.y() ],
        }
    }
}
