use mystic::astro::chart::*;
use mystic::astro::planets::*;
use std::collections::HashMap;
use sweet::*;

sweet! {

	before{
		let day = Y2000Day::new(2000,1,1);

	}
	test "all" {
		let chart = Chart::new(day).positions;

		// for (body,zodiac) in chart.iter(){
		// 	let deg = zodiac.sign_angle.floor();
		// 	let min = (zodiac.sign_angle % 1.0 * 60.0).floor();
		// 	println!("{:?}: {:?} - {:?},{:?}\t\t{:?}", body,zodiac.sign.sign, deg,min,zodiac.sign_angle);
		// }

		//acurate to cafeastrology to within 2 minutes, could be they are ceil and im floor
		expect(chart[&Body::Sun].sign_angle).to_be(9.872532293896843)?;
		expect(chart[&Body::Moon].sign_angle).to_be(7.320173004879664)?;
		expect(chart[&Body::Mercury].sign_angle).to_be(1.1300987894853036)?;
		expect(chart[&Body::Venus].sign_angle).to_be(0.975554039728352)?;
		expect(chart[&Body::Mars].sign_angle).to_be(27.58870720890896)?;
		expect(chart[&Body::Jupiter].sign_angle).to_be(25.254946738346774)?;
		expect(chart[&Body::Saturn].sign_angle).to_be(10.39831323519355)?;
		expect(chart[&Body::Uranus].sign_angle).to_be(14.780314883457436)?;
		expect(chart[&Body::Neptune].sign_angle).to_be(3.1730962372338807)?;
		expect(chart[&Body::Pluto].sign_angle).to_be(11.4404557412276)?;

	}
}
/*
cafeastrology

Sun		Capricorn	9°52'
Moon		Scorpio	7°18'
Mercury		Capricorn	1°08'
Venus		Sagittarius	0°58'
Mars		Aquarius	27°35'
Jupiter		Aries	25°15'
Saturn		Taurus	10°24'	R
Uranus		Aquarius	14°48'
Neptune		Aquarius	3°12'
Pluto		Sagittarius	11°28'
*/
