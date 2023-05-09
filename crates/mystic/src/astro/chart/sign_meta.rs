use super::*;
use std::{collections::HashMap, f64::consts::TAU};
use strum::EnumCount;

pub const ZODIAC_ANGLE: f64 = TAU / 12.0;

pub fn get_zodiac() -> HashMap<Sign, SignMeta> {
	let mut map = HashMap::with_capacity(Sign::COUNT);
	map.insert(Sign::Aries, SignMeta::ARIES);
	map.insert(Sign::Taurus, SignMeta::TAURUS);
	map.insert(Sign::Gemini, SignMeta::GEMINI);
	map.insert(Sign::Cancer, SignMeta::CANCER);
	map.insert(Sign::Leo, SignMeta::LEO);
	map.insert(Sign::Virgo, SignMeta::VIRGO);
	map.insert(Sign::Libra, SignMeta::LIBRA);
	map.insert(Sign::Scorpio, SignMeta::SCORPIO);
	map.insert(Sign::Sagittarius, SignMeta::SAGITTARIUS);
	map.insert(Sign::Capricorn, SignMeta::CAPRICORN);
	map.insert(Sign::Aquarius, SignMeta::AQUARIUS);
	map.insert(Sign::Pisces, SignMeta::PISCES);
	map
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct SignMeta {
	pub sign: Sign,
	pub polarity: Polarity,
	pub element: Element,
	pub mode: Mode,
}




impl Into<SignMeta> for Sign {
	fn into(self) -> SignMeta {
		match self {
			Sign::Aries => SignMeta::ARIES,
			Sign::Taurus => SignMeta::TAURUS,
			Sign::Gemini => SignMeta::GEMINI,
			Sign::Cancer => SignMeta::CANCER,
			Sign::Leo => SignMeta::LEO,
			Sign::Virgo => SignMeta::VIRGO,
			Sign::Libra => SignMeta::LIBRA,
			Sign::Scorpio => SignMeta::SCORPIO,
			Sign::Sagittarius => SignMeta::SAGITTARIUS,
			Sign::Capricorn => SignMeta::CAPRICORN,
			Sign::Aquarius => SignMeta::AQUARIUS,
			Sign::Pisces => SignMeta::PISCES,
		}
	}
}

impl TryFrom<usize> for SignMeta {
	type Error = ();

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(SignMeta::ARIES),
			1 => Ok(SignMeta::TAURUS),
			2 => Ok(SignMeta::GEMINI),
			3 => Ok(SignMeta::CANCER),
			4 => Ok(SignMeta::LEO),
			5 => Ok(SignMeta::VIRGO),
			6 => Ok(SignMeta::LIBRA),
			7 => Ok(SignMeta::SCORPIO),
			8 => Ok(SignMeta::SAGITTARIUS),
			9 => Ok(SignMeta::CAPRICORN),
			10 => Ok(SignMeta::AQUARIUS),
			11 => Ok(SignMeta::PISCES),
			_ => Err(()),
		}
	}
}
