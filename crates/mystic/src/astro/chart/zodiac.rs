use std::collections::HashMap;

use strum_macros::EnumCount;




pub fn get_zodiac() -> HashMap<Sign, Zodiac> {
	let map = HashMap::with_capacity(Sign::COUNT);
	map.insert(
		Sign::Aries,
		Zodiac {
			sign: Sign::Aries,
			polarity: Polarity::Positive,
			element: Element::Fire,
			mode: Mode::Cardinal,
		},
	);
	map.insert(
		Sign::Taurus,
		Zodiac {
			sign: Sign::Taurus,
			polarity: Polarity::Negative,
			element: Element::Earth,
			mode: Mode::Fixed,
		},
	);
	map.insert(
		Sign::Gemini,
		Zodiac {
			sign: Sign::Gemini,
			polarity: Polarity::Positive,
			element: Element::Air,
			mode: Mode::Mutable,
		},
	);

	map.insert(
		Sign::Cancer,
		Zodiac {
			sign: Sign::Cancer,
			polarity: Polarity::Negative,
			element: Element::Water,
			mode: Mode::Cardinal,
		},
	);

	map.insert(
		Sign::Leo,
		Zodiac {
			sign: Sign::Leo,
			polarity: Polarity::Positive,
			element: Element::Fire,
			mode: Mode::Fixed,
		},
	);

	map.insert(
		Sign::Virgo,
		Zodiac {
			sign: Sign::Virgo,
			polarity: Polarity::Negative,
			element: Element::Earth,
			mode: Mode::Mutable,
		},
	);

	map.insert(
		Sign::Libra,
		Zodiac {
			sign: Sign::Libra,
			polarity: Polarity::Positive,
			element: Element::Air,
			mode: Mode::Cardinal,
		},
	);

	map.insert(
		Sign::Scorpio,
		Zodiac {
			sign: Sign::Scorpio,
			polarity: Polarity::Negative,
			element: Element::Water,
			mode: Mode::Fixed,
		},
	);

	map.insert(
		Sign::Sagittarius,
		Zodiac {
			sign: Sign::Sagittarius,
			polarity: Polarity::Positive,
			element: Element::Fire,
			mode: Mode::Mutable,
		},
	);

	map.insert(
		Sign::Capricorn,
		Zodiac {
			sign: Sign::Capricorn,
			polarity: Polarity::Negative,
			element: Element::Earth,
			mode: Mode::Cardinal,
		},
	);

	map.insert(
		Sign::Aquarius,
		Zodiac {
			sign: Sign::Aquarius,
			polarity: Polarity::Positive,
			element: Element::Air,
			mode: Mode::Fixed,
		},
	);

	map.insert(
		Sign::Pisces,
		Zodiac {
			sign: Sign::Pisces,
			polarity: Polarity::Negative,
			element: Element::Water,
			mode: Mode::Mutable,
		},
	);

	map
}


pub struct Zodiac {
	pub sign: Sign,
	pub polarity: Polarity,
	pub element: Element,
	pub mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumCount)]
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

pub enum Polarity {
	Positive,
	Negative,
}

pub enum Element {
	Fire,
	Earth,
	Air,
	Water,
}

pub enum Mode {
	Cardinal,
	Fixed,
	Mutable,
}
