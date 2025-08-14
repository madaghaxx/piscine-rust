pub trait Scalar: _ {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}