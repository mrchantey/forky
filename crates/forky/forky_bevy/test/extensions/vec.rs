use bevy_math::prelude::*;
use forky_bevy::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	for i in 0..10 {
		let val = Vec3::random_in_cube();
		expect(val.length()).to_be_less_than(2.)?;
		// println!("random_in_cube: {val}");
	}
	for i in 0..10 {
		let val = Vec3::random_on_sphere();
		expect(val.length()).to_be_close_to(1.)?;
		// println!("random_on_sphere: {val}");
	}
	for i in 0..10 {
		let val = Vec3::random_in_sphere();
		expect(val.length()).to_be_less_than(1.)?;
		// println!("random_in_sphere: {val}");
	}

	Ok(())
}
