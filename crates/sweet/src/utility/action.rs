pub trait ActionRun {
	fn run(&mut self) {}
}

// impl Action {}


pub struct Action<F>
where
	F: FnMut() -> (),
{
	pub func: F,
}


impl<F> Action<F>
where
	F: FnMut() -> (),
{
	pub fn new(func: F) -> Action<F> { Action { func } }
}

impl<F> ActionRun for Action<F>
where
	F: FnMut() -> (),
{
	fn run(&mut self) {
		// let f = self.func;

		(self.func)();
	}
}
