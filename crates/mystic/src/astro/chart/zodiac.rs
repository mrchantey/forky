use super::*;
use std::{collections::HashMap, f64::consts::TAU};
use strum::EnumCount;
use strum_macros::EnumCount;

pub const ZODIAC_ANGLE: f64 = TAU / 12.0;

pub fn get_zodiac() -> HashMap<Sign, SignMeta> {
	let mut map = HashMap::with_capacity(Sign::COUNT);
	map.insert(Sign::Aries, constants::ARIES);
	map.insert(Sign::Taurus, constants::TAURUS);
	map.insert(Sign::Gemini, constants::GEMINI);
	map.insert(Sign::Cancer, constants::CANCER);
	map.insert(Sign::Leo, constants::LEO);
	map.insert(Sign::Virgo, constants::VIRGO);
	map.insert(Sign::Libra, constants::LIBRA);
	map.insert(Sign::Scorpio, constants::SCORPIO);
	map.insert(Sign::Sagittarius, constants::SAGITTARIUS);
	map.insert(Sign::Capricorn, constants::CAPRICORN);
	map.insert(Sign::Aquarius, constants::AQUARIUS);
	map.insert(Sign::Pisces, constants::PISCES);
	map
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SignMeta {
	pub sign: Sign,
	pub polarity: Polarity,
	pub element: Element,
	pub mode: Mode,
}


impl SignMeta {
	pub fn from_index(index: usize) -> Self {
		match index {
			0 => constants::ARIES,
			1 => constants::TAURUS,
			2 => constants::GEMINI,
			3 => constants::CANCER,
			4 => constants::LEO,
			5 => constants::VIRGO,
			6 => constants::LIBRA,
			7 => constants::SCORPIO,
			8 => constants::SAGITTARIUS,
			9 => constants::CAPRICORN,
			10 => constants::AQUARIUS,
			11 => constants::PISCES,
			_ => panic!("Invalid index"),
		}
	}
}


impl Into<SignMeta> for Sign {
	fn into(self) -> SignMeta {
		match self {
			Sign::Aries => constants::ARIES,
			Sign::Taurus => constants::TAURUS,
			Sign::Gemini => constants::GEMINI,
			Sign::Cancer => constants::CANCER,
			Sign::Leo => constants::LEO,
			Sign::Virgo => constants::VIRGO,
			Sign::Libra => constants::LIBRA,
			Sign::Scorpio => constants::SCORPIO,
			Sign::Sagittarius => constants::SAGITTARIUS,
			Sign::Capricorn => constants::CAPRICORN,
			Sign::Aquarius => constants::AQUARIUS,
			Sign::Pisces => constants::PISCES,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
pub enum Sign {
	Aries,
	Taurus,
	Gemini,
	Cancer,
	Leo,
	Virgo,
	Libra,
	Scorpio,
	Sagittarius,
	Capricorn,
	Aquarius,
	Pisces,
}

impl Sign {
	pub fn meta(&self) -> SignMeta { self.clone().into() }
	// TODO these could be faster with math
	pub fn element(&self) -> Element { self.meta().element }
	pub fn polarity(&self) -> Polarity { self.meta().polarity }
	pub fn mode(&self) -> Mode { self.meta().mode }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Polarity {
	Positive,
	Negative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
	Fire,
	Earth,
	Air,
	Water,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
	Cardinal,
	Fixed,
	Mutable,
}
