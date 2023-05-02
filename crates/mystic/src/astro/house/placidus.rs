use super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{GeographicCoords, Y2000Day, DEG2RAD},
};
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct PlacidusHouse(pub [ZodiacPosition; 12]);

impl PlacidusHouse {
	pub fn new(day: Y2000Day, position: &GeographicCoords) -> Self {
		let mut houses: [ZodiacPosition; 12] = Default::default();

		let (i, iv, vii, x) = get_i_iv_vii_x(day, position);

		//TODO Diurnal arc

		houses[0] = ZodiacPosition::from_angle(i * DEG2RAD);
		// houses[1] = ZodiacPosition::from_angle(ii * DEG2RAD);
		// houses[2] = ZodiacPosition::from_angle(iii * DEG2RAD);
		houses[3] = ZodiacPosition::from_angle(iv * DEG2RAD);
		// houses[4] = ZodiacPosition::from_angle(v * DEG2RAD);
		// houses[5] = ZodiacPosition::from_angle(vi * DEG2RAD);
		houses[6] = ZodiacPosition::from_angle(vii * DEG2RAD);
		// houses[7] = ZodiacPosition::from_angle(viii * DEG2RAD);
		// houses[8] = ZodiacPosition::from_angle(ix * DEG2RAD);
		houses[9] = ZodiacPosition::from_angle(x * DEG2RAD);
		// houses[10] = ZodiacPosition::from_angle(xi * DEG2RAD);
		// houses[11] = ZodiacPosition::from_angle(xii * DEG2RAD);

		Self(houses)
	}
}

impl HouseSystem for PlacidusHouse {
	fn from_index(&self, index: usize) -> &ZodiacPosition { &self[index] }
}

impl std::fmt::Display for PlacidusHouse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();
		for i in 0..12 {
			s.push_str(
				format!(
					"{:?}\t\t{}\n",
					House::from_index(i),
					self.from_index(i)
				)
				.as_str(),
			);
		}
		write!(f, "{}", s)
	}
}
