#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        Matrix(matrix)
    }
}







































use std::ops::{ Add, Sub, Mul, Div };

pub trait Scalar: Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Copy +
    Clone +
    PartialEq +
    Sized
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}
