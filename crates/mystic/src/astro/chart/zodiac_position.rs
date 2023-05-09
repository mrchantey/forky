use super::*;
use crate::astro::planets::{
	deg_min_sec_str, wrap_rad, EclipticCoords, GeoCoords, DEG2RAD, RAD2DEG,
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ZodiacPosition {
	pub sign: SignMeta,
	/// The position of the planet in the sign, in degrees
	pub sign_angle: f64,
	/// The position of the planet in the zodiac, in degrees
	pub zodiac_angle: f64,
}

impl ZodiacPosition {
	pub fn from_deg(degrees: f64) -> Self { Self::from_rad(degrees * DEG2RAD) }
	pub fn from_rad(radians: f64) -> Self {
		let zodiac_angle = wrap_rad(radians);
		let zodiac_index = ((zodiac_angle) / ZODIAC_ANGLE).floor() as usize;
		let sign_angle = zodiac_angle % ZODIAC_ANGLE;
		Self {
			sign: SignMeta::try_from(zodiac_index).unwrap(),
			sign_angle: sign_angle * RAD2DEG,
			zodiac_angle: zodiac_angle * RAD2DEG,
		}
	}
}

impl From<GeoCoords> for ZodiacPosition {
	fn from(coords: GeoCoords) -> Self { Self::from_rad(coords.flat_angle()) }
}
impl From<&GeoCoords> for ZodiacPosition {
	fn from(coords: &GeoCoords) -> Self { Self::from_rad(coords.flat_angle()) }
}
impl From<EclipticCoords> for ZodiacPosition {
	fn from(coords: EclipticCoords) -> Self {
		Self::from_rad(coords.longitude * DEG2RAD)
	}
}
impl From<&EclipticCoords> for ZodiacPosition {
	fn from(coords: &EclipticCoords) -> Self {
		Self::from_rad(coords.longitude * DEG2RAD)
	}
}

impl std::fmt::Display for ZodiacPosition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:?} - {}",
			self.sign.sign,
			deg_min_sec_str(self.sign_angle),
		)
	}
}
