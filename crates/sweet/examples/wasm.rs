#![feature(imported_main)]
use anyhow::Result;
use leptos::*;
pub use sweet::*;

#[sweet_test]
fn passes() -> Result<()> {
	mount_to_body(|cx| {
		view! {cx,
			<div>
			"hello world!"
			</div>
		}
	});
	expect(true).to_be_true()?;
	Ok(())
}
// #[sweet_test]
// fn fails() -> Result<()> {
// 	expect(true).to_be_true()?;
// 	Ok(())
// }
