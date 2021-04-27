pub mod matrix2x2;
pub mod matrix3x3;
pub mod matrix4x4;

#[derive(Debug, Clone)]
pub struct InverseMatrixError<'a>(&'a str);
