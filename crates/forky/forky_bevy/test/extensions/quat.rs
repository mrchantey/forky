use bevy_app::prelude::*;
use bevy_math::prelude::*;
use forky_bevy::prelude::*;
use sweet::*;

sweet! {
	it "works" {
		let v = Quat::from_right();
		expect(v.forward().x).to_be_close_to(1.)?;
		let v = Quat::from_left();
		expect(v.forward().x).to_be_close_to(-1.)?;
		let v = Quat::from_up();
		expect(v.forward().y).to_be_close_to(1.)?;
		let v = Quat::from_down();
		expect(v.forward().y).to_be_close_to(-1.)?;
		let v = Quat::from_forward();
		expect(v.forward().z).to_be_close_to(1.)?;
		let v = Quat::from_back();
		expect(v.forward().z).to_be_close_to(-1.)?;
	}
}
