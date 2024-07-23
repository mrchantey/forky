use forky_bevy::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let app = AppRes::new();
	let app = app.borrow_mut();
	expect(app.world().contains_non_send::<AppRes>()).to_be_true()?;
	Ok(())
}
