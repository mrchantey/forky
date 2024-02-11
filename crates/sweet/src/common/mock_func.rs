use std::sync::Arc;
use std::sync::Mutex;



fn _empty_func(_: ()) {}

pub fn mock_trigger() -> MockFunc<(), (), fn(())> { MockFunc::new(_empty_func) }
pub fn mock_func<I, O, F: Fn(I) -> O>(func: F) -> MockFunc<I, O, F> {
	MockFunc::new(func)
}

#[derive(Debug, Clone)]
pub struct MockFunc<I, O, F> {
	pub called: Arc<Mutex<Vec<O>>>,
	pub func: F,
	pub _phantom: std::marker::PhantomData<I>,
}


impl<I, O, F: Fn(I) -> O> MockFunc<I, O, F> {
	pub fn new(func: F) -> Self {
		Self {
			called: Default::default(),
			func,
			_phantom: std::marker::PhantomData,
		}
	}
	pub fn call(&self, input: I) {
		let output = (self.func)(input);
		self.called.lock().unwrap().push(output);
	}
}
impl<I: Default, O, F: Fn(I) -> O> MockFunc<I, O, F> {
	pub fn call0(&self) {
		let output = (self.func)(I::default());
		self.called.lock().unwrap().push(output);
	}
}

impl<I, O: Clone, F: Fn(I) -> O> MockFunc<I, O, F> {
	pub fn call_and_get(&self, input: I) -> O {
		let output = (self.func)(input);
		self.called.lock().unwrap().push(output.clone());
		output
	}
}
