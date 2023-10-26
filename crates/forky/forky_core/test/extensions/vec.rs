use forky_core::*;
use sweet::*;

sweet! {
	test "sorted" {
		let v = vec![0,2,1].sorted();
		expect(v.len()).to_be(3)?;
		expect(v[0]).to_be(0)?;
		expect(v[1]).to_be(1)?;
		expect(v[2]).to_be(2)?;
		// expect(true).to_be(false)?;

	}
	test "first" {
		let v = vec![0,2,1];
		let v = v._first();
		expect(*v).to_be(0)?;
		// expect(true).to_be(false)?;

	}
}
