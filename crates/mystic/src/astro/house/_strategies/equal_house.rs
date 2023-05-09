use super::super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{EclipticCoords, GeographicCoords, Y2000Day},
};

/// Single Longitude Method. Even spacing between houses from ascendant.
pub struct EqualHouse;

impl HouseSystemStrategy for EqualHouse {
	fn calculate(
		day: Y2000Day,
		position: &GeographicCoords,
		houses: &mut [ZodiacPosition; 12],
	) {
		let delta = 360. * 1. / 12.;
		let asc = EclipticCoords::eastern_horizon_intersect(day, &position);
		for i in 0..12 {
			houses[i] =
				ZodiacPosition::from_deg(asc.longitude + delta * i as f64);
		}
	}
}
