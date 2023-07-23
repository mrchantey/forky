#![feature(imported_main)]
use anyhow::Result;
use forky_core::*;
pub use sweet::*;

#[sweet_test]
fn foobar() -> Result<()> {
	log!("woah!");
	expect(true).to_be_true()?;
	Ok(())
}
#[sweet_test]
fn fails() -> Result<()> {
	expect(true).to_be_false()?;
	Ok(())
}
// #[sweet_test]
// fn panics() -> Result<()> {
// 	panic!("oh no!");
// 	Ok(())
// }
// #[sweet_test]
// fn it_sleeps() -> Result<()> {
// 	std::thread::sleep(std::time::Duration::from_millis(1000));
// 	expect(true).to_be_true()?;
// 	Ok(())
// }

// #[sweet_test]
// fn fizzboo() -> Result<()> {
// 	expect(true).to_be_false()?;
// 	Ok(())
// }
