use std::ops::*;
pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> + Default {}

impl<T: Copy + Add<Output = Self> + Mul<Output = Self> + Default> Scalar for T {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar > Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Vec::new();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result.push(*a + *b);
        }
        Self(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> T {
        let mut sum = T::default();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let product = *a * *b;
            sum = sum + product;
        }
        sum
    }
}
