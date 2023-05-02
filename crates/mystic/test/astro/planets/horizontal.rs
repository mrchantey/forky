use mystic::astro::{chart::{Chart, ZodiacPosition}, planets::*};
use sweet::*;
//http://cosinekitty.com/solar_system.html
const DAY: Y2000Day = Y2000Day::FIRST_JAN_2000;
const POS: GeographicCoords = GeographicCoords::SYDNEY;

sweet! {
	test "rect"{
		let hor = ecliptic_positions::ecliptic_position(DAY,Planet::Sun)
			.to_geo(DAY).to_equatorial_with_correction(DAY,&POS)
			.to_horizontal_with_correction(DAY, &POS);
		let rect = hor.to_rectangular();
		let hor2 = rect.to_horizontal();
		let epsilon = 1e-15;
		expect(hor.azimuth).to_be_close_to_with_epsilon(hor2.azimuth,epsilon)?;
		expect(hor.altitude).to_be_close_to_with_epsilon(hor2.altitude,epsilon)?;
		expect(hor.distance).to_be_close_to_with_epsilon(hor2.distance,epsilon)?;
	}

	test "equatorial to horizontal" {

		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Sun).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
			.to_be(HorizontalCoords::new(75.12365173365818, 61.98656204739518, 0.9833154966487052))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Moon).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(300.07502490086614, 49.157797563279765,  0.0026780889984901832))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Mercury).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(66.78236910802838, 70.12273699439677, 1.4131239881531124))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Venus).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(322.69619665691374, 71.16919059405858,  1.1344370259553465))?;
		// expect(ecliptic_positions::ecliptic_position(day,Body::Earth).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
		// 				.to_be(HorizontalCoords::new(75.12365173365818, 61.98656204739518, 0.))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Jupiter).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(108.95152055426674, -39.78962278881888, 4.6151445225085475))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Saturn).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(117.98438806245959, -53.682900476557826, 8.643001983947073))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Uranus).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(91.87972534995909, 28.863136485083526, 20.720022432032987))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Neptune).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(87.44110044923087, 39.91789975719002, 31.004809501352373))?;
		expect(ecliptic_positions::ecliptic_position(DAY,Planet::Pluto).to_geo(DAY).to_equatorial_with_correction(DAY,&POS).to_horizontal_with_correction(DAY,&POS))
						.to_be(HorizontalCoords::new(0.5834303819348269, 67.52303658204691, 31.106821859132157))?;
	}


	test "horizontal vs horizontal with correction"{
		let eq = ecliptic_positions::ecliptic_position(DAY,Planet::Sun).to_geo(DAY).to_equatorial_with_correction(DAY,&POS);
		let hor = eq.to_horizontal_with_correction(DAY, &POS);
		let hor2 = eq.to_horizontal(DAY, &POS);
		let epsilon = 1e-13;
		expect(hor.azimuth).to_be_close_to_with_epsilon(hor2.azimuth,epsilon)?;
		expect(hor.altitude).to_be_close_to_with_epsilon(hor2.altitude,epsilon)?;
		expect(hor.distance).to_be(hor2.distance)?;
	}
	test "horizontal to equatorial"{
		let eq = ecliptic_positions::ecliptic_position(DAY,Planet::Sun).to_geo(DAY).to_equatorial_with_correction(DAY,&POS);
		let hor = eq.to_horizontal(DAY, &POS);
		let eq2 = hor.to_equatorial(DAY, &POS);
		let epsilon = 1e-14;
		expect(eq.right_ascention).to_be_close_to_with_epsilon(eq2.right_ascention,epsilon)?;
		expect(eq.declination).to_be_close_to_with_epsilon(eq2.declination,epsilon)?;
		expect(eq.radius).to_be_close_to_with_epsilon(eq2.radius,epsilon)?;
	}
}
