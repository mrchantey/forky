

use super::*;
use crate::astro::planets::{rad2deg, wrapRad, RectangluarCoords};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZodiacPosition {
	pub sign: Zodiac,
	/// The position of the planet in the sign, in degrees
	pub sign_angle: f64,
	/// The position of the planet in the zodiac, in degrees
	pub zodiac_angle: f64,
}


impl From<&RectangluarCoords> for ZodiacPosition {
	fn from(value: &RectangluarCoords) -> Self {
		// let delta_theta =
		let zodiac_angle = wrapRad(value.flat_angle());

		let zodiac_index = ((zodiac_angle) / ZODIAC_ANGLE).floor() as usize;
		let sign = Zodiac::from_index(zodiac_index);
		let sign_angle = zodiac_angle % ZODIAC_ANGLE;
		Self {
			sign,
			sign_angle: sign_angle * rad2deg,
			zodiac_angle: zodiac_angle * rad2deg,
		}
	}
}
