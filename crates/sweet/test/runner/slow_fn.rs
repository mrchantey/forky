use sweet::*;
//slow
fn long_fn() -> f32 {
	let mut a = 3290.;
	for _x in 0..100000 {
		for _y in 0..10000 {
			a = f32::sqrt(a);
		}
	}
	a
}

describe!("slow fn", |s| {
	s.skip().it("works slowly", || {
		let a = long_fn();
		expect(a).not().to_be(0.)?;
		Ok(())
	});
});
