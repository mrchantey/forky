use mystic::astro::planets::*;
use sweet::*;



sweet! {
	it "works" {

		let system = SolarSystem::new(Y2000Day::new(2000,1,1));
		// let system = SolarSystem::new(Y2000Day::new(1990,4,19));
		// let system = SolarSystem::new(Y2000Day(0.));

		let sun_pos = system.bodies[&Body::Sun].ecliptic_rect;
		expect(sun_pos.x).to_be(0.1606934)?;
		expect(sun_pos.y).to_be(-0.9701041)?;
		// let moon_pos = system.bodies[&Body::Moon].ecliptic_rect;
		// expect(moon_pos.x).to_be(-0.0022583)?;
		// expect(moon_pos.y).to_be(-0.0013977)?;

	}
}
