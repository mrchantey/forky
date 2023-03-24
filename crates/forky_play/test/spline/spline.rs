use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
sweet! {
	test "spline acceleration - linear" {

		let spline = Spline::Linear(LinearSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
		});
		
		expect(spline.acceleration(0.,Vec3::DOWN)).to_be(1.)?;
		expect(spline.acceleration(0.,Vec3::UP)).to_be(-1.)?;
		expect(spline.acceleration(0.,Vec3::RIGHT)).to_be(0.)?;
		expect(spline.acceleration(0.,Vec3::new(1.,1.,0.).normalize()))
			.to_be_close_to(-0.70)?;
		expect(spline.acceleration(0.,Vec3::new(2.,2.,0.)))
			.to_be_close_to(-2.)?;
		expect(spline.acceleration(0.,Vec3::new(0.,-2.,0.)))
			.to_be_close_to(2.)?;

		
	}
	test "spline acceleration - quadratic" {
		
		let spline = Spline::Quadratic(QuadraticSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
			p2: Vec3::RIGHT,
		});
		expect(spline.acceleration(0.,Vec3::DOWN)).to_be(1.)?;
		expect(spline.acceleration(0.5,Vec3::DOWN)).to_be_close_to(0.7)?;
		expect(spline.acceleration(1.,Vec3::DOWN)).to_be(0.)?;
		expect(spline.acceleration(0.5,Vec3::new(1.,-2.,0.)))
			.to_be_close_to(2.1)?;
		expect(spline.acceleration(0.5,Vec3::new(-2.,2.,0.)))
			.to_be_close_to(-2.8)?;
	}
	test "spline acceleration - cubic" {
		
		let spline = Spline::Cubic(CubicSpline{
			p0: Vec3::new(0.,1.,0.),
			p1: Vec3::new(0.,0.,0.),
			p2: Vec3::new(1.,0.,0.),
			p3: Vec3::new(1.,1.,0.),
		});
		expect(spline.acceleration(0.,Vec3::DOWN)).to_be(1.)?;
		expect(spline.acceleration(0.25,Vec3::DOWN)).to_be_close_to(0.8)?;
		expect(spline.acceleration(0.5,Vec3::DOWN)).to_be(0.)?;
		expect(spline.acceleration(0.75,Vec3::DOWN)).to_be_close_to(-0.8)?;
		expect(spline.acceleration(1.,Vec3::DOWN)).to_be(-1.)?;

	}

}
