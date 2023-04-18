use mystic::astro::chart::*;
use mystic::astro::planets::*;
use sweet::*;

sweet! {
	it "works" {

		let day = Y2000Day::new(1992, 2, 20).add_utc_time(10,0, 0);
		let chart = Chart::new(day);

		for item in chart.positions.iter() {
			println!("{:?}", item);
		}
		// println!("{:?}", chart);
		// expect(true).to_be(false)?;

	}
}
