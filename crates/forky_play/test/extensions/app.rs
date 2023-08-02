use forky_play::*;
use sweet::*;

sweet! {
	test "app_res" {
		let app = AppRes::new();
		let app = app.borrow_mut();
		expect(app.world.contains_non_send::<AppRes>()).to_be_true()?;
	}
}
