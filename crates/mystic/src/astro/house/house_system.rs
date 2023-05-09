use super::*;
use crate::astro::{
	chart::ZodiacPosition,
	planets::{GeographicCoords, Y2000Day},
};
use derive_deref::{Deref, DerefMut};


//https://www.costarastrology.com/how-does-astrology-work/house-systems\
//https://www.alicesparklykat.com/articles/3/Houses_in_Astrology_Part_2/

//Astrology Podcast - House Division Calculations Explained - https://youtu.be/6ZmqOgdxkK4
/*
- Whole Sign
- Equal Houses
- Porphyry
- Alcabitius
- Campanus
- Regiomontanus
- Placidus / Hour Line
*/



pub trait HouseSystemStrategy {
	fn calculate(
		day: Y2000Day,
		position: &GeographicCoords,
		houses: &mut [ZodiacPosition; 12],
	);
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut)]
pub struct HouseSystem(pub [ZodiacPosition; 12]);

impl HouseSystem {
	pub fn new<T: HouseSystemStrategy>(
		day: Y2000Day,
		position: &GeographicCoords,
	) -> Self {
		let mut value = Self::default();
		value.calculate::<T>(day, position);
		value
	}

	pub fn calculate<T: HouseSystemStrategy>(
		&mut self,
		day: Y2000Day,
		position: &GeographicCoords,
	) {
		T::calculate(day, position, &mut *self)
	}

	pub fn get(&self, house: House) -> &ZodiacPosition {
		&self[Into::<usize>::into(house)]
	}
}

impl std::fmt::Display for HouseSystem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();
		for (i, pos) in self.iter().enumerate() {
			s.push_str(
				format!("{:?}\t\t{}\n", House::try_from(i).unwrap(), pos)
					.as_str(),
			);
		}
		write!(f, "{}", s)
	}
}
