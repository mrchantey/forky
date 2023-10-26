use forky_core::*;
use sweet::*;


sweet! {
	test "log!" {
		// log!(1 " is better than " 0);
	}


	test "tern!" {
		let a = tern!(0 < 1; "foo";"bar");
		expect(a).to_be("foo")?;
	}
}
