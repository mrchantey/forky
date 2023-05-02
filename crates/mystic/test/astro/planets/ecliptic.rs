use mystic::{
	astro::{
		chart::ZodiacPosition,
		planets::{
			EclipticCoords, GeographicCoords, HorizontalCoords, Y2000Day,
		},
	},
	*,
};
use sweet::*;

sweet! {

	test "eastern horizon ecliptic"{
		let day = Y2000Day::FIRST_JAN_2000;
		let position = GeographicCoords::GREENWICH;
		let coords = EclipticCoords::eastern_horizon_intersect(day, &position);
		//TODO find a reference site, this is just a snapshot
		expect(coords.longitude).to_be(187.0320245239699)?;
	}
}
