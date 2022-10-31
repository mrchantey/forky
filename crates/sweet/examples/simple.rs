#![feature(imported_main)]
pub use sweet::*;

sweet! {"banana"
	let mut a = 1;
	let b = 2;
	test "pizza"{
		a = a + 1;
		expect(a).to_be(2)?;
	}
}


// fn main() {
// 	print!("howdy");
// }
