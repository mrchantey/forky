use std::time::Duration;

use extend::ext;
use forky_core::DurationExt;
use time::OffsetDateTime;

use super::MILLIS2DAYS;



#[ext]
pub impl OffsetDateTime {
	fn to_julian_day_with_time(&self) -> f64 {
		let julian = self.to_julian_day(); //ignores time
		let (h, m, s) = self.to_hms(); //ignores date
		let time = Duration::from_hms(h as u64, m as u64, s as u64).as_millis();
		julian as f64 + time as f64 * MILLIS2DAYS
	}
}
