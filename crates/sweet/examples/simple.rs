use sweet::*;

sweet! {"banana"
	let mut a = 1;
	let b = 2;
	test skip "pizza"{
		expect(a).to_be(a)?;
	}
}


fn main() {
	// let a = 2;
	// let a = (1 < 2) ?;
	// let a = tn!(a > a; a : a);
	// tn!(1 < 2 ? 1 : 2);
}
