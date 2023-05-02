use std::{collections::HashMap, fmt::Display};

use super::*;
use crate::astro::{
	planets::{Planet, Y2000Day},
	*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Chart {
	pub positions: HashMap<Planet, ZodiacPosition>,
}

impl Chart {
	pub fn new(day: Y2000Day) -> Self {
		let positions = planets::SolarSystem::geocentric_ecliptic(day)
			.iter()
			.map(|(key, body)| (*key, body.into()))
			.collect();

		Chart { positions }
	}

	// #[rustfmt::skip]
	// 	pub fn get_ascendant(day:Y2000Day,position:&GeographicCoords)->ZodiacPosition{
	// 	let geo = HorizontalCoords::get_eastern_ecliptic_intersection(day, position)
	// 		.to_equatorial(day, position)
	// 		.to_geo(day);
	// 		(&geo).into()
	// 	}
}

impl Display for Chart {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let positions = self
			.positions
			.iter()
			.map(|(planet, position)| format!("{:?}: {}", planet, position))
			.collect::<Vec<String>>()
			.join("\n");

		f.write_fmt(format_args!("{}", positions))
	}
}
