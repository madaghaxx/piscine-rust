use std::ops::{ Add };
use lalgebra_scalar::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector(
            self.0
                .into_iter()
                .zip(other.0)
                .map(|(a, b)| a + b)
                .collect()
        )
    }
}

impl<T: Scalar + Default> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = T::default();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result = result + *a * *b;
        }
        Some(result)
    }
}
