use super::super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{
		unsigned_angle_between_d, wrap_deg, EclipticCoords, GeographicCoords,
		Y2000Day,
	},
};

/// Dual Longitude Method. Even spacing between quadrants
pub struct PorphyryHouse;

impl HouseSystemStrategy for PorphyryHouse {
	fn calculate(
		day: Y2000Day,
		position: &GeographicCoords,
		houses: &mut [ZodiacPosition; 12],
	) {
		let (i, iv, vii, x) = get_i_iv_vii_x(day, position);

		let q1 = unsigned_angle_between_d(i, iv) / 3.;
		let ii = i + 1. * q1;
		let iii = i + 2. * q1;
		let viii = vii + 1. * q1;
		let ix = vii + 2. * q1;
		let q2 = unsigned_angle_between_d(iv, vii) / 3.;
		let v = iv + 1. * q2;
		let vi = iv + 2. * q2;
		let xi = x + 1. * q2;
		let xii = x + 2. * q2;

		houses[0] = ZodiacPosition::from_deg(i);
		houses[1] = ZodiacPosition::from_deg(ii);
		houses[2] = ZodiacPosition::from_deg(iii);
		houses[3] = ZodiacPosition::from_deg(iv);
		houses[4] = ZodiacPosition::from_deg(v);
		houses[5] = ZodiacPosition::from_deg(vi);
		houses[6] = ZodiacPosition::from_deg(vii);
		houses[7] = ZodiacPosition::from_deg(viii);
		houses[8] = ZodiacPosition::from_deg(ix);
		houses[9] = ZodiacPosition::from_deg(x);
		houses[10] = ZodiacPosition::from_deg(xi);
		houses[11] = ZodiacPosition::from_deg(xii);
	}
}


pub fn get_i_iv_vii_x(
	day: Y2000Day,
	position: &GeographicCoords,
) -> (f64, f64, f64, f64) {
	let i = EclipticCoords::eastern_horizon_intersect(day, &position).longitude;
	let vii = wrap_deg(i + 180.);
	let x = EclipticCoords::meridian_intersect(day, &position).longitude;
	let iv = wrap_deg(x + 180.);
	(i, iv, vii, x)
}
