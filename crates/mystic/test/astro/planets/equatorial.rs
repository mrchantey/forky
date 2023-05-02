use mystic::astro::planets::*;
use sweet::*;
//http://cosinekitty.com/solar_system.html
sweet! {
	test "geocentric to equatorial with correction" {
		let day = Y2000Day::FIRST_JAN_2000;
		let position = GeographicCoords::SYDNEY;
		//100% accurate with cosinekitty
		expect(ecliptic_positions::ecliptic_position(day,Planet::Sun).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(18.71611585187057, -23.071385654536318, 0.9833154966487052))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Moon).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(14.417484465170674, -8.596795754328033, 0.0026780889984901832))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Mercury).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(18.08274238082684, -24.380635981533466, 1.4131239881531124))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Venus).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(15.952410840694727, -18.31761643397223, 1.1344370259553465))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Earth).to_geo(day).to_equatorial_with_correction(day,&position).to_string().as_str())
			.to_be(EquatorialCoords::new(f64::NAN,f64::NAN,0.0).to_string().as_str())?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Jupiter).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(1.5913431171387373, 8.59473659329149, 4.6151445225085475))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Saturn).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(2.584700779979909, 12.608724369868964, 8.643001983947073))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Uranus).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(21.163085734920678, -17.02975121085124, 20.720022432032987))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Neptune).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(20.36083801993554, -19.218838313378413, 31.004809501352373))?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Pluto).to_geo(day).to_equatorial_with_correction(day,&position))
			.to_be(EquatorialCoords::new(16.760333917021597, -11.392798471005001, 31.106821859132157))?;
	}

	test "equatorial to geocentric"{
		let day = Y2000Day::FIRST_JAN_2000;

		let pos = ecliptic_positions::ecliptic_position(day,Planet::Sun).to_geo(day);
		let eq = pos.to_equatorial(day);
		let pos2 = eq.to_geo(day);
		let epsilon = 1e-15;
		expect(pos.x).to_be_close_to_with_epsilon(pos2.x, epsilon)?;
		expect(pos.y).to_be_close_to_with_epsilon(pos2.y, epsilon)?;
		expect(pos.z).to_be_close_to_with_epsilon(pos2.z, epsilon)?;
	}
	test "equatorial to geocentric with correction"{
		let day = Y2000Day::FIRST_JAN_2000;
		let position = GeographicCoords::SYDNEY;

		let pos = ecliptic_positions::ecliptic_position(day,Planet::Sun).to_geo(day);
		let eq = pos.to_equatorial_with_correction(day,&position);
		let pos2 = eq.to_geo_with_correction(day,&position);
		let epsilon = 1e-9;
		expect(pos.x).to_be_close_to_with_epsilon(pos2.x, epsilon)?;
		expect(pos.y).to_be_close_to_with_epsilon(pos2.y, epsilon)?;
		expect(pos.z).to_be_close_to_with_epsilon(pos2.z, epsilon)?;
	}
}
