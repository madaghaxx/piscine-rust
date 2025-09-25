pub use matrix::*;
pub use std::ops::Mul;
#[derive(Debug, PartialEq,Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        let mut res = vec![];
        for i in self.0.iter() {
            res.push(i[n]);
        }
        res
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }
}

impl<T: Mul<Output = T> + Copy> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Option<Self> {
        let mut res = vec![];
        for (v1, v2) in self.0.iter().zip(other.0.iter()) {
            if v1.len() != v2.len() {
                return None;
            }
            let mut temp = vec![];
            for (vv1, vv2) in v1.iter().zip(v2.iter()) {
                temp.push(*vv1 * *vv2);
            }
            res.push(temp);
        }
        Some(Matrix(res))
    }
}
