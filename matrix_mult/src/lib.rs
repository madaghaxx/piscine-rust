use std::ops::{ Add, Sub, Mul, Div };

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = Vec::new();
        for row in &self.0 {
            res.push(row[n].clone());
        }
        res
    }
}
impl<T: Mul<Output = T> + Add<Output = T> + Copy + Default> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut new_row = Vec::new();
            for j in 0..other.number_of_cols() {
                let mut sum = T::default();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k] * other.0[k][j];
                }
                new_row.push(sum);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}

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
