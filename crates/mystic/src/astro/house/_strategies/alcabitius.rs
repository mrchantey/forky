use super::super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{EclipticCoords, GeographicCoords, Y2000Day},
};

/// Standard Method??.
pub struct AlcabitiusHouse;

impl HouseSystemStrategy for AlcabitiusHouse {
	fn calculate(
		day: Y2000Day,
		position: &GeographicCoords,
		houses: &mut [ZodiacPosition; 12],
	) {
		let i =
			EclipticCoords::eastern_horizon_intersect(day, &position).longitude;
		let x = EclipticCoords::meridian_intersect(day, &position).longitude;

		houses[0] = ZodiacPosition::from_deg(i);
		houses[9] = ZodiacPosition::from_deg(x);
	}
}
