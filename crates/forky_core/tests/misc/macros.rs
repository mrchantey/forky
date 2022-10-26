use forky_core::testing::*;
use forky_core::*;


describe!("macros", |s| {
	s.test("tern!", || {
		let a = tern!(0 < 1; "foo";"bar");
		expect(a).to_be("bar")?;
		Ok(())
	})
});
