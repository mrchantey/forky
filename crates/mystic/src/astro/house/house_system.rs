use super::*;
use crate::astro::chart::ZodiacPosition;


//https://www.costarastrology.com/how-does-astrology-work/house-systems\
//https://www.alicesparklykat.com/articles/3/Houses_in_Astrology_Part_2/
pub trait HouseSystem {
	// fn new(day: Y2000Day, position: &GeographicCoords) -> Self;
	fn from_index(&self, index: usize) -> &ZodiacPosition;

	fn get(&self, house: House) -> &ZodiacPosition {
		self.from_index(house.to_index())
	}
}



impl std::fmt::Display for dyn HouseSystem {
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
