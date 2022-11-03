use sweet::*;

sweet! {

	let mut a = 1;
	it "gets" {
		expect(a).to_be(1)?;
		a = a + 1;
	}
	it "mutates" {
		expect(a).to_be(2)?;
	}
}
