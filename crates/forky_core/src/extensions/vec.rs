use extend::ext;
use std::slice::SliceIndex;

#[ext(name = VecXOrd)]
pub impl<T: Ord> Vec<T> {
	fn sorted(mut self) -> Self {
		self.sort();
		self
	}
}

#[ext(name = VecXDefault)]
pub impl<T: Default + Clone> Vec<T> {
	fn from_len(len: usize) -> Self { vec![T::default(); len] }
}
#[ext(name = VecXInto)]
pub impl<T: Into<U>, U> Vec<T> {
	fn into_vec(self) -> Vec<U> { self.into_iter().map(|x| x.into()).collect() }
}


#[ext(name = VecX)]
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
	fn remove_first_element(&mut self, element: T) -> bool
	where
		T: PartialEq,
	{
		let index = self.iter().position(|x| *x == element);
		if let Some(index) = index {
			self.remove(index);
			true
		} else {
			false
		}
	}
}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let v = vec![0, 2, 1].sorted();
		expect(v.len()).to_be(3);
		expect(v[0]).to_be(0);
		expect(v[1]).to_be(1);
		expect(v[2]).to_be(2);
	}

	#[test]
	fn first() {
		let v = vec![0, 2, 1];
		let v = v._first();
		expect(*v).to_be(0);
	}
}
