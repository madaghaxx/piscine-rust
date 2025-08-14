use std::ops::*;
pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> {}

impl<T: Copy + Add<Output = Self> + Mul<Output = Self>> Scalar for T {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result: Vector<T> = Vector(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x.clone() + y.clone())
                .collect()
        );
        Some(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = None;
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let product = *a * *b;
            sum = match sum {
                None => Some(product),
                Some(s) => Some(s + product),
            };
        }
        sum
    }
}
