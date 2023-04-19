use mystic::astro::planets::*;
use sweet::*;
//http://cosinekitty.com/solar_system.html
sweet! {
	it "works" {
		let day = DAY_1_JAN_2000;
		let position = GEOGRAPHIC_COORDS_SYDNEY;
		
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Sun).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 18.71611585187057, declination: -23.071385654536318, radius: 0.9833154966487052 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Moon).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 14.417484465170674, declination: -8.596795754328033, radius: 0.0026780889984901832 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Mercury).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 18.08274238082684, declination: -24.380635981533466, radius: 1.4131239881531124 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Venus).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 15.952410840694727, declination: -18.31761643397223, radius: 1.1344370259553465 })?;
		// expect(ecliptic_positions::ecliptic_position_geo(day,Body::Earth).to_equatorial(day,position))
		// 	.to_be(EquatorialCoords { right_ascention: f64::NAN, declination: f64::NAN, radius: 0.0 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Jupiter).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 1.5913431171387373, declination: 8.59473659329149, radius: 4.6151445225085475 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Saturn).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 2.584700779979909, declination: 12.608724369868964, radius: 8.643001983947073 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Uranus).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 21.163085734920678, declination: -17.02975121085124, radius: 20.720022432032987 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Neptune).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 20.36083801993554, declination: -19.218838313378413, radius: 31.004809501352373 })?;
		expect(ecliptic_positions::ecliptic_position_geo(day,Body::Pluto).to_equatorial(day,position))
			.to_be(EquatorialCoords { right_ascention: 16.760333917021597, declination: -11.392798471005001, radius: 31.106821859132157 })?;
	}
}
