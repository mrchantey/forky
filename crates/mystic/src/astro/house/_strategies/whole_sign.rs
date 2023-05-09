use super::super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{GeographicCoords, Y2000Day},
};

/// Floored even spacing between houses from ascendant. Asc floats in first house, MC floats above.
pub struct WholeSignHouse;

impl HouseSystemStrategy for WholeSignHouse {
	fn calculate(
		day: Y2000Day,
		position: &GeographicCoords,
		houses: &mut [ZodiacPosition; 12],
	) {
		EqualHouse::calculate(day, position, houses);
		for house in houses {
			house.sign_angle = 0.;
			house.zodiac_angle = house.zodiac_angle - house.zodiac_angle % 30.;
		}
	}
}


// is this a thing? keep reading ascendant every two hours
// let delta = 1. / 12.;
// for i in 0..12 {
// 	let day = Y2000Day(*day + delta * i as f64);
// 	houses[i] =
// 		EclipticCoords::eastern_horizon_intersect(day, &position)
// 			.into();
// }
