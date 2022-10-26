use forky_core::testing::*;
use forky_core::*;
fn main() { run(); }

pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}


fn long_fn() -> f32 {
	let mut a = 3290.;
	for _x in 0..100000 {
		for _y in 0..10000 {
			a = f32::sqrt(a);
		}
	}
	a
}

describe!("test runner", |s| {
	fn setup() -> Vec2 { Vec2 { x: 0., y: 0. } }

	s.test("once", || {
		let mut v = setup();
		v.x = v.x + 1.;
		expect(v.x).to_be(1.)?;
		Ok(())
	});

	s.test("twice", || {
		let val = long_fn();
		log!(val);
		let mut v = setup();
		v.x = v.x + 1.;
		println!("here in twice");
		expect(v.x).to_be(0.)?;
		Ok(())
	});

	s.it("three times a lady", || {
		let mut v = setup();
		v.x = v.x + 1.;
		expect(v.x).to_be(1.)?;
		Ok(())
	});
});
