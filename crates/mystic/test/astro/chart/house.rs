use mystic::astro::chart::*;
use mystic::astro::house::*;
use mystic::astro::planets::*;
use std::collections::HashMap;
use sweet::*;


const DAY: Y2000Day = Y2000Day::FIRST_JAN_2000;
const POS: GeographicCoords = GeographicCoords::GREENWICH;

sweet! {

	test "house - WholeSign"{

		let houses = HouseSystem::new::<WholeSignHouse>(DAY,&POS);
		// println!("Whole Sign: \n{}",houses);
	}
	test "house - EqualSign"{

		let houses = HouseSystem::new::<EqualHouse>(DAY,&POS);
		// println!("Equal Sign: \n{}",houses);
	}
	test "house - Porphyry"{

		let houses = HouseSystem::new::<PorphyryHouse>(DAY,&POS);
		// println!("Porphyry:\n{}",houses);
	}
	// test "ascendant2"{
	// 	// let day = Y2000Day::FIRST_JAN_2000;
	// 	// let position = GeographicCoords::GREENWICH;
	// 	let day = Y2000Day::new(1992,2,20).add_utc_time(9, 0, 0);
	// 	let position = GeographicCoords::SYDNEY;
	// 	let coords = EclipticCoords::eastern_horizon_intersect(day, &position);

	// 	let ascendant:ZodiacPosition = (&coords).into();
	// 	println!("{}",*coords);
	// 	println!("{}",ascendant);
	// }
}
/*

Placidus

I ASC		Libra	7°02'

II		Scorpio	1°44'
III		Sagittarius	2°36'

IV		Capricorn	9°10'

V		Aquarius	14°45'
VI		Pisces	14°01'

VII		Aries	7°02'

VIII		Taurus	1°44'
IX		Gemini	2°36'

X MC		Cancer	9°10'

XI		Leo	14°45'
XII		Virgo	14°01'
*/
