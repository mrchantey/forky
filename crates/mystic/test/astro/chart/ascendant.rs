use mystic::astro::chart::*;
use mystic::astro::house::*;
use mystic::astro::planets::*;
use std::collections::HashMap;
use sweet::*;


const DAY: Y2000Day = Y2000Day::FIRST_JAN_2000;
const POS: GeographicCoords = GeographicCoords::GREENWICH;

sweet! {

	test "ascendant"{
		let coords = EclipticCoords::eastern_horizon_intersect(DAY, &POS);
		let ascendant:ZodiacPosition = (&coords).into();
		expect(ascendant.to_string().as_str()).to_be("Libra - 7° 1' 55\"")?;
	}

	test "meridian"{
		let coords = EclipticCoords::meridian_intersect(DAY, &POS);
		let ascendant:ZodiacPosition = (&coords).into();
		expect(ascendant.to_string().as_str()).to_be("Cancer - 9° 9' 36\"")?;
	}
	test "ascendant midheaven"{
		for i in 0..12{
			let day = Y2000Day::new(2000,1 + i as u8,1).add_utc_time(0, 0, 0);
			let pos = GeographicCoords::new(0., 0.,0.);
		let time = EclipticCoords::ascendant_midheaven_time(day, &pos);
		// let mc1 = EclipticCoords::meridian_intersect(day, &pos);
		let day2 = day.add_hours(time);
		let mc1 = EclipticCoords::meridian_intersect(day, &pos).longitude;
		let mc2 = EclipticCoords::eastern_horizon_intersect(day2, &pos).longitude - 180.;
		println!("mc1: {:?}, mc2: {:?}",mc1,mc2);
		// println!("mc2: {:?}",mc2);
		// println!("day: {}, day2: {}",day,day2);

		}
		// let day = Y2000Day::new(1992,2,20).add_utc_time(9, 0, 0);
		// let time = EclipticCoords::ascendant_midheaven_time(day, &POS);
		// println!("{}",time);
		// let ascendant: ZodiacPosition = (&coords).into();
		// expect(ascendant.to_string().as_str()).to_be("Cancer - 9° 9' 36\"")?;
	}
}
