use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
sweet! {
	test "spline acceleration" {

		let spline = Spline::Linear(LinearSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
		});

		let tangent = spline.tangent(0.);
		expect(spline.acceleration(0.,Vec3::DOWN)).to_be(1.)?;
		expect(spline.acceleration(0.,Vec3::UP)).to_be(-1.)?;
		expect(spline.acceleration(0.,Vec3::RIGHT)).to_be(0.)?;
		expect(spline.acceleration(0.,Vec3::new(1.,1.,0.).normalize()))
			.to_be_close_to(-0.1)?;
	}

}
