use std::collections::HashMap;

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
}
