use bevy::prelude::*;
use sweet::*;


sweet! {
	it "works" {
		expect(&Vec3::ZERO).to_be_close_to(Vec3::ZERO)?;
		expect(&Vec3::ZERO).not().to_be_close_to(Vec3::ONE)?;

	}
}
