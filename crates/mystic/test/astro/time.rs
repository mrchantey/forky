use mystic::astro::planets::*;
use sweet::*;
use time::{Date, Month, OffsetDateTime};

sweet! {
	it "works" {

		expect(*Y2000Day::new(2000, 1, 1)).to_be(0.0)?;
		expect(*Y2000Day::new(2000, 1, 1).add_utc_time(12, 0, 0)).to_be(0.5)?;
		expect(*Y2000Day::new(2001, 1, 1)).to_be(366.)?;
		expect(*Y2000Day::now()).to_be_greater_than(8000.)?;
		expect(*Y2000Day::now()).to_be_less_than(9000.)?;


	}
}
