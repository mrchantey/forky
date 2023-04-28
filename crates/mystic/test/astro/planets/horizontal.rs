use mystic::astro::planets::*;
use sweet::*;

sweet! {
	it "works" {
		let day = FIRST_JAN_2000_DAY;
		let position = GEOGRAPHIC_COORDS_SYDNEY;

		expect(ecliptic_positions::ecliptic_position(day,Planet::Sun).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
			.to_be(HorizontalCoords { azimuth: 75.12365173365818, altitude: 61.98656204739518 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Moon).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 300.07502490086614, altitude: 49.157797563279765 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Mercury).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 66.78236910802838, altitude: 70.12273699439677 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Venus).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 322.69619665691374, altitude: 71.16919059405858 })?;
		// expect(ecliptic_positions::ecliptic_position(day,Body::Earth).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
		// 				.to_be(HorizontalCoords { azimuth: 75.12365173365818, altitude: 61.98656204739518 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Jupiter).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 108.95152055426674, altitude: -39.78962278881888 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Saturn).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 117.98438806245959, altitude: -53.682900476557826 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Uranus).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 91.87972534995909, altitude: 28.863136485083526 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Neptune).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 87.44110044923087, altitude: 39.91789975719002 })?;
		expect(ecliptic_positions::ecliptic_position(day,Planet::Pluto).to_geo(day).to_equatorial(day,&position).to_horizontal(day,&position))
						.to_be(HorizontalCoords { azimuth: 0.5834303819348269, altitude: 67.52303658204691 })?;
	}
}
