use mystic::astro::planets::*;
use sweet::*;

sweet! {
	test "sun" {
		let day = Y2000Day::new(2000,1,1);

		let pos_geo = ecliptic_positions::sun_geo(day);
		expect(pos_geo.x).to_be(0.16859614540661194)?;
		expect(pos_geo.y).to_be(-0.9687542029346362)?;
		let pos_helio = ecliptic_positions::sun_helio();
		expect(pos_helio.x).to_be(0.)?;
		expect(pos_helio.y).to_be(0.)?;
	}
}
