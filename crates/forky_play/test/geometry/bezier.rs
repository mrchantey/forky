use forky_play::bezier;
use sweet::*;
sweet! {
	test "linear" {
		expect(bezier::linear(0.0, 10.0, 0.5)).to_be_close_to( 5.0)?;
		expect(bezier::linear(0.0, -10.0, 0.25)).to_be_close_to(-2.5)?;
		expect(bezier::linear(-5.0, 5.0, 1.0)).to_be_close_to(5.0)?;
	}

	test "quadratic" {
		expect(bezier::quadratic(0.0, 5.0, 10.0, 0.5)).to_be_close_to(5.0)?;
		expect(bezier::quadratic(0.0, 2.0, 4.0, 0.25)).to_be_close_to(1.0)?;
		expect(bezier::quadratic(-1.0, 0.0, 1.0, 1.0)).to_be_close_to(1.0)?;
	}

	test "cubic" {
			expect(bezier::cubic(0.0, 2.0, 4.0, 6.0, 0.5)).to_be_close_to(3.0)?;
			expect(bezier::cubic(0.0, 1.0, -1.0, 0.0, 0.25)).to_be_close_to(0.28125)?;
			expect(bezier::cubic(-1.0, 0.0, 1.0, 2.0, 1.0)).to_be_close_to(2.0)?;
	}

	test "tangent_linear" {
		expect(bezier::tangent_linear(0.0, 5.0)).to_be_close_to(5.)?;
		expect(bezier::tangent_linear(0.5, 2.0)).to_be_close_to(1.5)?;
		expect(bezier::tangent_linear(-1.0, -5.0)).to_be_close_to(-4.0)?;
}

	test "tangent_quadratic" {
			expect(bezier::tangent_quadratic(0.0, 5.0, 10.0, 0.5)).to_be_close_to(10.0)?;
			expect(bezier::tangent_quadratic(0.0, 2.0, 4.0, 0.25)).to_be_close_to(4.0)?;
			expect(bezier::tangent_quadratic(-1.0, 0.0, 1.0, 1.0)).to_be_close_to(2.0)?;
	}

	test "tangent_cubic" {
			expect(bezier::tangent_cubic(0.0, 2.0, 4.0, 6.0, 0.5)).to_be_close_to(6.0)?;
			expect(bezier::tangent_cubic(0.0, 1.0, -1.0, 0.0, 0.25)).to_be_close_to(-0.375)?;
			expect(bezier::tangent_cubic(-1.0, 0.0, 1.0, 2.0, 1.0)).to_be_close_to(3.0)?;
	}
}
