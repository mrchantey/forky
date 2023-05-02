use crate::astro::{
	chart::ZodiacPosition,
	planets::{EclipticCoords, GeographicCoords, Y2000Day},
};
use derive_deref::{Deref, DerefMut};


#[derive(Debug, Deref, DerefMut)]
/// Porphyry house system, even spacing between I, IV, VII, and X.
pub struct WholeSignHouse(pub [ZodiacPosition; 12]);

impl WholeSignHouse {
	pub fn new(day: Y2000Day, position: &GeographicCoords) -> Self {
		let mut houses: [ZodiacPosition; 12] = Default::default();
		let delta = 1. / 12.;
		for i in 0..12 {
			let day = Y2000Day(*day + delta * i as f64);
			houses[i] =
				EclipticCoords::eastern_horizon_intersect(day, &position)
					.into();
		}
		Self(houses)
	}
}
