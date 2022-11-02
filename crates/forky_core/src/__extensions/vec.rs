use std::slice::SliceIndex;

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
	fn _first(&self) -> &T { self.first().unwrap() }
	fn _pop(&mut self) -> T { self.pop().unwrap() }
	fn _get<T2>(&self, index: T2) -> &<T2 as SliceIndex<[T]>>::Output
	where
		T2: SliceIndex<[T]>,
	{
		self.get(index).unwrap()
	}
	fn _get_mut<T2>(&mut self, index: T2) -> &<T2 as SliceIndex<[T]>>::Output
	where
		T2: SliceIndex<[T]>,
	{
		self.get_mut(index).unwrap()
	}
}
