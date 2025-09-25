pub use matrix::*;
pub use std::ops::Mul;
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = vec![];
        for i in self.0.iter() {
            res.push(i[n]);
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