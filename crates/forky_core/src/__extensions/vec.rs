use extend::ext;

#[ext(name = OrderableVecExt)]
pub impl<T: Ord> Vec<T> {
	fn sorted(mut self) -> Self {
			self.sort();
			self
	}
}

#[ext(name = GenericVecExt)]
pub impl<T> Vec<T> {
	fn _first(&self) -> &T {
		self.first().unwrap()
	}
}