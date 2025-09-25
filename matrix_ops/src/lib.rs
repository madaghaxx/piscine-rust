pub use matrix::*;
use std::ops::{ Add, Sub };
#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        let mut res = vec![];
        for (v1, v2) in self.0.iter().zip(other.0.iter()) {
            if v1.len() != v2.len() {
                return None;
            }
            let mut temp = vec![];
            for (vv1, vv2) in v1.iter().zip(v2.iter()) {
                temp.push(*vv1 + *vv2);
            }
            res.push(temp);
        }
        Some(Matrix(res))
    }
}


impl<T: Sub<Output = T> + Copy> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Option<Self> {
        let mut res = vec![];
        for (v1, v2) in self.0.iter().zip(other.0.iter()) {
            if v1.len() != v2.len() {
                return None;
            }
            let mut temp = vec![];
            for (vv1, vv2) in v1.iter().zip(v2.iter()) {
                temp.push(*vv1 - *vv2);
            }
            res.push(temp);
        }
        Some(Matrix(res))
    }
}
