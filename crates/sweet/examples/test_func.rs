#![feature(imported_main)]
use forky_core::*;
pub use sweet::*;

test! {"works", skip,
	log!("woah!");
	expect(true).to_be_true()?;
}
// case! {
// 	expect(true).to_be_false()?;
// }
