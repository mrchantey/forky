use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
sweet! {


	test "spline"{
		let t = 0.21233;
		let p0 = Vec3::new(32.3,903.0,3893.0);
		let p1 = Vec3::new(33.,90399.0,383.0);
		let p2 = Vec3::new(32.3,3.0,38.0);
		let p3 = Vec3::new(3.,90.0,93.0);

		let linear = LinearSpline::new(p0,p1);
		let linear2 = Spline::Linear(linear);
		let quadratic = QuadraticSpline::new(p0,p1,p2);
		let quadratic2 = Spline::Quadratic(quadratic);
		let cubic = CubicSpline::new(p0,p1,p2,p3);
		let cubic2 = Spline::Cubic(cubic);


		expect(linear.position(t)).to_be(linear2.position(t))?;
		expect(quadratic.position(t)).to_be(quadratic2.position(t))?;
		expect(cubic.position(t)).to_be(cubic2.position(t))?;
		expect(linear.tangent(t)).to_be(linear2.tangent(t))?;
		expect(quadratic.tangent(t)).to_be(quadratic2.tangent(t))?;
		expect(cubic.tangent(t)).to_be(cubic2.tangent(t))?;
	}




	test "acceleration - linear" {

		let spline = LinearSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
		};

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
	test "acceleration - quadratic" {

		let spline = QuadraticSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
			p2: Vec3::RIGHT,
		};
		expect(spline.acceleration(0.,Vec3::DOWN)).to_be(1.)?;
		expect(spline.acceleration(0.5,Vec3::DOWN)).to_be_close_to(0.7)?;
		expect(spline.acceleration(1.,Vec3::DOWN)).to_be(0.)?;
		expect(spline.acceleration(0.5,Vec3::new(1.,-2.,0.)))
			.to_be_close_to(2.1)?;
		expect(spline.acceleration(0.5,Vec3::new(-2.,2.,0.)))
			.to_be_close_to(-2.8)?;
	}
	test "acceleration - cubic" {

		let spline = CubicSpline{
			p0: Vec3::new(0.,1.,0.),
			p1: Vec3::new(0.,0.,0.),
			p2: Vec3::new(1.,0.,0.),
			p3: Vec3::new(1.,1.,0.),
		};
		expect(spline.acceleration(0.,Vec3::DOWN)).to_be(1.)?;
		expect(spline.acceleration(0.25,Vec3::DOWN)).to_be_close_to(0.8)?;
		expect(spline.acceleration(0.5,Vec3::DOWN)).to_be(0.)?;
		expect(spline.acceleration(0.75,Vec3::DOWN)).to_be_close_to(-0.8)?;
		expect(spline.acceleration(1.,Vec3::DOWN)).to_be(-1.)?;

	}

	test "total length"{
		let spline = LinearSpline{
			p0: Vec3::new(0.,0.,0.),
			p1: Vec3::new(10.,0.,0.),
		};
		expect(spline.total_length(0)).to_be(10.)?;
		expect(spline.total_length(1000)).to_be(10.)?;

		let spline = CubicSpline{
			p0: Vec3::new(0.,1.,0.),
			p1: Vec3::new(0.,0.,0.),
			p2: Vec3::new(1.,0.,0.),
			p3: Vec3::new(1.,1.,0.),
		};
		expect(spline.total_length(0)).to_be(1.)?;
		expect(spline.total_length(2)).to_be_close_to(1.91)?;
		expect(spline.total_length(10)).to_be_close_to(1.99)?;

		expect(spline.get_lengths(0)[0]).to_be(0.)?;
		expect(spline.get_lengths(0)[1]).to_be(1.)?;
		expect(spline.get_lengths(1)[0]).to_be(0.)?;
		expect(spline.get_lengths(1)[1]).to_be_close_to(0.9)?;
		expect(spline.get_lengths(1)[2]).to_be_close_to(1.8)?;
	}

	test "normal"{
		let spline = LinearSpline{
			p0: Vec3::ZERO,
			p1: Vec3::RIGHT,
		};
		expect(spline.tangent(0.)).to_be(Vec3::RIGHT)?;
		expect(spline.tangent(1.)).to_be(Vec3::RIGHT)?;
		expect(spline.normal(0.)).to_be(Vec3::Z)?;
		expect(spline.normal(1.)).to_be(Vec3::Z)?;

		let spline = LinearSpline{
			p0: Vec3::ZERO,
			p1: Vec3::UP,
		};
		expect(spline.tangent(0.)).to_be(Vec3::UP)?;
		expect(spline.tangent(1.)).to_be(Vec3::UP)?;
		expect(spline.normal(0.)).to_be(Vec3::Z_NEG)?;
		expect(spline.normal(1.)).to_be(Vec3::Z_NEG)?;


		let spline = Spline::Linear(LinearSpline{
			p0: Vec3::ZERO,
			p1: Vec3::Z,
		});
		expect(spline.tangent(0.)).to_be(Vec3::Z)?;
		expect(spline.tangent(1.)).to_be(Vec3::Z)?;
		expect(spline.normal(0.)).to_be(Vec3::LEFT)?;
		expect(spline.normal(1.)).to_be(Vec3::LEFT)?;
	}

}
