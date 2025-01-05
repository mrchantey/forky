#[derive(Default)]
pub struct ForkyEvent<T> {
	pub listeners: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> ForkyEvent<T> {
	pub fn new() -> Self {
		Self {
			listeners: Vec::new(),
		}
	}

	pub fn trigger(&mut self, val: &T) {
		for listener in self.listeners.iter_mut() {
			listener(val);
		}
	}
	pub fn add_listener(&mut self, listener: impl 'static + FnMut(&T)) {
		self.listeners.push(Box::new(listener));
	}

	pub fn clear(&mut self) { self.listeners.clear(); }
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let mut e = ForkyEvent::default();
		let func = mock_bucket::<i32>();
		let func2 = func.clone();
		e.add_listener(move |val| func2(*val));
		e.trigger(&3);

		expect(&func).to_have_returned_with(3);
	}
}
