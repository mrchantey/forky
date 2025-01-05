use super::*;

pub fn wrap(value: f64, wrap: f64) -> f64 {
	value - f64::floor(value / wrap) * wrap
}

//https://stackoverflow.com/questions/1878907/how-can-i-find-the-difference-between-two-angles
/// currently always negative?
pub fn angle_between(a: f64, b: f64) -> f64 { wrap(a - b + PI, TAU) - PI }
pub fn unsigned_angle_between(a: f64, b: f64) -> f64 {
	f64::abs(angle_between(a, b))
}
/// currently always negative?
pub fn angle_between_d(a: f64, b: f64) -> f64 {
	wrap(a - b + 180., 360.) - 180.
}
pub fn unsigned_angle_between_d(a: f64, b: f64) -> f64 {
	f64::abs(angle_between_d(a, b))
}


#[cfg(test)]
mod test {
	use super::*;
	use sweet::prelude::*;

	#[test]
	fn wrap_function() {
		expect(wrap(3.5, 2.0)).to_be_close_to(1.5);
		expect(wrap(-1.5, 2.0)).to_be_close_to(0.5);
		expect(wrap(7.0, TAU)).to_be_close_to(wrap(7.0 - TAU, TAU));
		expect(wrap(5.0 * PI, TAU)).to_be_close_to(PI);
	}

	#[test]
	fn angle_between_radians() {
		expect(angle_between(0.0, 0.0)).to_be_close_to(0.0);
		expect(angle_between(PI, 0.0)).to_be_close_to(-PI);
		expect(angle_between(0.0, PI)).to_be_close_to(-PI);
		expect(angle_between(3.0 * PI, PI)).to_be_close_to(0.0);
		expect(angle_between(-PI / 2.0, PI / 2.0)).to_be_close_to(-PI);
	}

	#[test]
	fn unsigned_angle_between_radians() {
		expect(unsigned_angle_between(0.0, 0.0)).to_be_close_to(0.0);
		expect(unsigned_angle_between(PI, 0.0)).to_be_close_to(PI);
		expect(unsigned_angle_between(0.0, PI)).to_be_close_to(PI);
		expect(unsigned_angle_between(TAU, 0.0)).to_be_close_to(0.0);
		expect(unsigned_angle_between(-PI / 2.0, PI / 2.0)).to_be_close_to(PI);
	}

	#[test]
	fn angle_between_degrees() {
		expect(angle_between_d(0.0, 0.0)).to_be_close_to(0.0);
		expect(angle_between_d(180.0, 0.0)).to_be_close_to(-180.0);
		expect(angle_between_d(0.0, 180.0)).to_be_close_to(-180.0);
		expect(angle_between_d(540.0, 180.0)).to_be_close_to(0.0);
		expect(angle_between_d(-90.0, 90.0)).to_be_close_to(-180.0);
	}

	#[test]
	fn unsigned_angle_between_degrees() {
		expect(unsigned_angle_between_d(0.0, 0.0)).to_be_close_to(0.0);
		expect(unsigned_angle_between_d(180.0, 0.0)).to_be_close_to(180.0);
		expect(unsigned_angle_between_d(0.0, 180.0)).to_be_close_to(180.0);
		expect(unsigned_angle_between_d(360.0, 0.0)).to_be_close_to(0.0);
		expect(unsigned_angle_between_d(-90.0, 90.0)).to_be_close_to(180.0);
	}
}
