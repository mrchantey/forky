pub trait SweetInto<T> {
	fn sweet_into(self) -> T;
}
pub trait SweetBorrow<T> {
	fn sweet_borrow(&self) -> T;
}

// impl<T,U> SweetInto<U> for T
// where
// 	T: std::ops::Deref<Target = U>,
// 	U:Clone
// {
// 	fn sweet_into(&self) -> U { (**self).clone() }
// }
// impl<T, U> SweetInto<U> for T
// where
// 	T: Into<U> + Clone,
// {
// 	fn sweet_into(&self) -> U { (*self).clone().into() }
// }
