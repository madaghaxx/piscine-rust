pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix{
            
        }
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
	}

	pub fn identity(n: usize) -> Matrix<T> {
	}
}