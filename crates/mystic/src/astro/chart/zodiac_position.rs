use super::*;
use crate::astro::planets::{wrap_rad, GeoCoords, RAD2DEG};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZodiacPosition {
	pub sign: Zodiac,
	/// The position of the planet in the sign, in degrees
	pub sign_angle: f64,
	/// The position of the planet in the zodiac, in degrees
	pub zodiac_angle: f64,
}


impl From<&GeoCoords> for ZodiacPosition {
	fn from(value: &GeoCoords) -> Self {
		// let delta_theta =
		let zodiac_angle = wrap_rad(value.flat_angle());

		let zodiac_index = ((zodiac_angle) / ZODIAC_ANGLE).floor() as usize;
		let sign = Zodiac::from_index(zodiac_index);
		let sign_angle = zodiac_angle % ZODIAC_ANGLE;
		Self {
			sign,
			sign_angle: sign_angle * RAD2DEG,
			zodiac_angle: zodiac_angle * RAD2DEG,
		}
	}
}
