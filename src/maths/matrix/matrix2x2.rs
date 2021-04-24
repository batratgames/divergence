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

    pub fn col(&self, i: usize) -> Vector2D {
        Vector2D::from((self.data[i], self.data[i + 2]))
    }

    pub fn row(&self, i: usize) -> Vector2D {
        Vector2D::from((self.data[2 * i], self.data[2 * i + 1]))
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, rhs: &Self) -> bool {
        self.data[0] == rhs.data[0] &&
        self.data[1] == rhs.data[1] &&
        self.data[2] == rhs.data[2] &&
        self.data[3] == rhs.data[3]
    }
}

impl Eq for Matrix2x2 {}

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

impl From<(Vector2D, Vector2D)> for Matrix2x2 {
    fn from(input: (Vector2D, Vector2D)) -> Matrix2x2 {
        Matrix2x2 {
            data: [ input.0.x(), input.1.x(), input.0.y(), input.1.y() ],
        }
    }
}

impl fmt::Debug for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]\n[{}, {}]", self.data[0], self.data[1], self.data[2], self.data[3])
    }
}

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.data[0], self.data[1], self.data[2], self.data[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        let test = Matrix2x2::zeros();
        let correct = Matrix2x2 { data: [ 0.0, 0.0, 0.0, 0.0 ] };

        assert_eq!(test, correct);
    }

    #[test]
    fn ones() {
        let test = Matrix2x2::ones();
        let correct = Matrix2x2 { data: [ 1.0, 1.0, 1.0, 1.0 ] };

        assert_eq!(test, correct);
    }

    #[test]
    fn identity() {
        let test = Matrix2x2::identity();
        let correct = Matrix2x2 { data: [ 1.0, 0.0, 0.0, 1.0 ] };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_float() {
        let test = Matrix2x2::from([1.0, 2.0, 3.0, 4.0]);
        let correct = Matrix2x2 { data: [ 1.0, 2.0, 3.0, 4.0 ] };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_integer() {
        let test = Matrix2x2::from([1, 2, 3, 4]);
        let correct = Matrix2x2 { data: [ 1.0, 2.0, 3.0, 4.0 ] };

        assert_eq!(test, correct);
    }

    #[test]
    fn from_vector() {
        let test = Matrix2x2::from((Vector2D::from((3, 4)), Vector2D::from((1, 2))));
        let correct = Matrix2x2::from([3, 1, 4, 2]);

        assert_eq!(test, correct);
    }

    #[test]
    fn determinant() {
        let test = Matrix2x2::from([3, 2, 3, 3]).determinant();
        let correct = 3.0;

        assert_eq!(test, correct);
    }

    #[test]
    fn col() {
        let matrix = Matrix2x2::from([1, 2, 3, 4]);
        let test1 = matrix.col(0);
        let test2 = matrix.col(1);
        let correct1 = Vector2D::from((1, 3));
        let correct2 = Vector2D::from((2, 4));

        assert_eq!(test1, correct1);
        assert_eq!(test2, correct2);
    }

    #[test]
    fn row() {
        let matrix = Matrix2x2::from([1, 2, 3, 4]);
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
        let test = Matrix2x2::from([1, 2, 3, 4]) + Matrix2x2::from([4, 3, 2, 1]);
        let correct = Matrix2x2::from([-3, -1, -1, -3]);

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
}
