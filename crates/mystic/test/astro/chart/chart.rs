use mystic::astro::chart::*;
use mystic::astro::planets::*;
use sweet::*;

sweet! {
	it "works" {
		let day = Y2000Day::new(2000,1,1);
		// let pos_geo = ecliptic_positions::sun_geo(day);
		// let zodiac:ZodiacPosition = (&pos_geo).into();
		// expect(zodiac.sign.sign).to_be(Sign::Capricorn)?;
		// expect(zodiac.sign_angle).to_be(9.872532293896843)?;

	}
}
