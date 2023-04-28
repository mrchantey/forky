use mystic::astro::planets::*;
use sweet::*;
use time::{Date, Month, OffsetDateTime};

sweet! {
	it "works" {

		expect(*Y2000Day::new(2000, 1, 1)).to_be(1.)?;
		expect(*Y2000Day::new(2000, 1, 1).add_utc_time(12, 0, 0)).to_be(1.5)?;
		expect(*Y2000Day::new(2001, 1, 1)).to_be(367.)?;
		expect(*Y2000Day::now()).to_be_greater_than(8000.)?;
		expect(*Y2000Day::now()).to_be_less_than(9000.)?;

		expect (*Y2000Day::from_unix_ms(FIRST_JAN_2000_MILLIS)).to_be(*FIRST_JAN_2000_DAY)?;


	}
}
