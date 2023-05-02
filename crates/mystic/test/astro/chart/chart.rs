use mystic::astro::chart::*;
use mystic::astro::planets::*;
use std::collections::HashMap;
use sweet::*;



sweet! {

	test "all" {
		let day:Y2000Day = Y2000Day::FIRST_JAN_2000;
		let chart = Chart::new(day).positions;

		// for (body,zodiac) in chart.iter(){
		// 	let deg = zodiac.sign_angle.floor();
		// 	let min = (zodiac.sign_angle % 1.0 * 60.0).floor();
		// 	println!("{:?}: {:?} - {:?},{:?}\t\t{:?}", body,zodiac.sign.sign, deg,min,zodiac.sign_angle);
		// }

		//acurate to cafeastrology to within 2 minutes, could be they are ceil and im floor
		expect(chart[&Planet::Sun].sign_angle).to_be(9.872532293896843)?;
		expect(chart[&Planet::Moon].sign_angle).to_be(7.320173004879664)?;
		expect(chart[&Planet::Mercury].sign_angle).to_be(1.1300987894853036)?;
		expect(chart[&Planet::Venus].sign_angle).to_be(0.975554039728352)?;
		expect(chart[&Planet::Mars].sign_angle).to_be(27.58870720890896)?;
		expect(chart[&Planet::Jupiter].sign_angle).to_be(25.254946738346774)?;
		expect(chart[&Planet::Saturn].sign_angle).to_be(10.39831323519355)?;
		expect(chart[&Planet::Uranus].sign_angle).to_be(14.780314883457436)?;
		expect(chart[&Planet::Neptune].sign_angle).to_be(3.1730962372338807)?;
		expect(chart[&Planet::Pluto].sign_angle).to_be(11.4404557412276)?;
	}

	test "utc millis"{

		let day1 = Y2000Day::from_unix_ms(1682647200000);
		let chart1 = Chart::new(day1);
		let day2 = Y2000Day::new(2023,4,28).add_utc_time(2, 0, 0);
		let chart2 = Chart::new(day2);
		// println!("{},{}",*day1,*day2);
		expect(chart1.positions[&Planet::Sun].zodiac_angle).to_be_close_to_with_epsilon(chart2.positions[&Planet::Sun].zodiac_angle,0.000000001)?;
	}

}
/*
cafeastrology

Placidus

I ASC		Libra	7°02'
II		Scorpio	1°44'
III		Sagittarius	2°36'
IV		Capricorn	9°10'
V		Aquarius	14°45'
VI		Pisces	14°01'
VII		Aries	7°02'
VIII		Taurus	1°44'
IX		Gemini	2°36'
X MC		Cancer	9°10'
XI		Leo	14°45'
XII		Virgo	14°01'
*/
