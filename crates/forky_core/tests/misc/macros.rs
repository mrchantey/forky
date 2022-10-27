use forky_core::*;
use forky_test::*;


describe!("macros", |s| {
	s.skip().test("log!", || {
		log!(1 " is better than " 0);
		Ok(())
	});


	s.test("tern!", || {
		let a = tern!(0 < 1; "foo";"bar");
		expect(a).to_be("foo")?;
		Ok(())
	})
});
