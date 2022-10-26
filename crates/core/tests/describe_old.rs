use core::testing::*;
use core::*;
fn main() { run(); }


fn do_thing<F>(mut f: F)
where
	F: FnMut(),
{
	f();
	f();
	f();
	f();
}

pub struct Suitie<T> {
	before_all: fn() -> T,
	test: fn(state: T),
}

pub trait Runnable {
	fn run(&self) -> i32;
}

impl<T> Runnable for Suitie<T> {
	fn run(&self) -> i32 {
		let state = (self.before_all)();


		// let foo = Vec<i32>::new();
		(self.test)(state);
		0
	}
}

pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}


fn do_it() -> Suitie<Vec2> {
	Suitie {
		before_all: (|| Vec2 { x: 1., y: 2. }),
		test: (|s| {
			expect(s.x).to_be(0.);
		}),
	}
}

describe!("test runner", |c| {
	let a = Vec2 { x: 0., y: 0. };

	// c.test("foobar", || {})
});
// describe!("test runner", |c| {
// 	let mut val = 0;

// 	do_thing(|| {
// 		val = val + 1;
// 		log!(val);
// 	});


// 	println!("building..");
// 	c.before_all(|| {
// 		// println!("before all");
// 	});
// 	c.before_each(|| {
// 		println!("before each");
// 	});
// 	c.after_all(|| {
// 		println!("after all");
// 	});
// 	c.after_each(|| {
// 		println!("after each");
// 	});

// 	c.it("takes 5 seconds", || {
// 		let mut a: f64 = 100000.;
// 		let len = 10000;
// 		for _ in 0..len {
// 			for _ in 0..len {
// 				a = f64::sqrt(a);
// 			}
// 		}
// 		log!(a);
// 	});

// 	c.test("all the steps", || {
// 		println!("test");
// 		expect(true).to_be_true();
// 	});
// });
