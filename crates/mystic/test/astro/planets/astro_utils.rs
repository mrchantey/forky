use mystic::astro::planets::*;
use sweet::*;

sweet! {
	test "utils" {

		expect(wrapDeg(360.)).to_be(0.)?;
		expect(wrapHours(24.)).to_be(0.)?;
		expect(wrapHours(25.)).to_be(1.)?;

		// expect(true).to_be(false)?;
		// expect(date_to_duration_since_2000(2000, 1, 1).as_secs()).to_be(0)?;
		// expect(time_to_duration(1, 1, 1).as_secs()).to_be(3661)?;
	}
}
