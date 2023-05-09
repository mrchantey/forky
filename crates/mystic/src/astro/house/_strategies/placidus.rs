use super::super::*;
use super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{GeographicCoords, Y2000Day, DEG2RAD},
};

pub struct PlacidusHouse;

//https://github.com/CruiserOne/Astrolog/blob/2676d1c63d22c66513c38f032a189e990ad2011e/matrix.cpp#L345-L387
//TODO
impl HouseSystemStrategy for PlacidusHouse {
	fn calculate(
		day: Y2000Day,
		position: &GeographicCoords,
		houses: &mut [ZodiacPosition; 12],
	) {
		let (i, iv, vii, x) = get_i_iv_vii_x(day, position);

		houses[0] = ZodiacPosition::from_deg(i);
		// houses[1] = ZodiacPosition::from_deg(ii);
		// houses[2] = ZodiacPosition::from_deg(iii);
		houses[3] = ZodiacPosition::from_deg(iv);
		// houses[4] = ZodiacPosition::from_deg(v);
		// houses[5] = ZodiacPosition::from_deg(vi);
		houses[6] = ZodiacPosition::from_deg(vii);
		// houses[7] = ZodiacPosition::from_deg(viii);
		// houses[8] = ZodiacPosition::from_deg(ix);
		houses[9] = ZodiacPosition::from_deg(x);
	}
}
