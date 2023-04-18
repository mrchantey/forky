use mystic::astro::planets::*;
use sweet::*;

sweet! {
	it "works" {

		//SYDNEY TEST
		let day = Y2000Day::new(2023, 4, 17).add_utc_time(8,3, 0);

		let syd_lon = 151.2093 * DEG2HOURS;
		let syd_lat = -33.8688;


		let system = SolarSystem::new(day).topocentric(syd_lat, syd_lon);

		// these are off by a few degrees from calculators
		expect(system[&Body::Sun].altitude).to_be_close_to(-0.07)?;
		expect(system[&Body::Sun].azimuth).to_be_close_to(269.9)?;

		//SCANDINAVIA TEST
		// let lon = 15. * deg2hours;
		// let lat = 60.;
		// let day = 0.0;
		// let day = getDaysFrom2000(1990, 4, 19, utcHour)
		//  (lon, lat, utc_hour);
	}
}

/*
SCANDINAVIA TEST VALUES
SUN
AZ 15.676702976646197
ALT -17.95700763078881

MOON
RA = 309.4881_deg
Decl = -19.0741_deg

SYDNEY TEST VALUES
10/07/2020 10:30:00 UT
TEST SUN
az 112°08'18" alt  -17°27'16"
TEST MOON
az 270°53'51" alt  -42°23'34"
*/
