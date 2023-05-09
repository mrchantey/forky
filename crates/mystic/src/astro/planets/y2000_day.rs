use super::{
	ecliptic_positions::OrbitalElements,
	offset_date_time_ext::OffsetDateTimeExt, *,
};
use derive_deref::{Deref, DerefMut};
use forky_core::DurationExt;
use std::time::Duration;
use time::{Date, Month, OffsetDateTime};

//millis, seconds, minutes, hours
// pub const millisInDay: u64 = 1000 * 60 * 60 * 24;
// // pub const millisInHour = 1000 * 60 * 60
// pub const y2000Millis: u64 = 946684800000;

/// julian date as of 0/Jan/2000 (31/Dec/1999) 0:00 UTC, ie
const JULIAN_Y2000: f64 = 2451544.;
// const JULIAN_Y2000: f64 = 2451545.;
const SECS_IN_DAY: f64 = 86400.;

pub const SECS2MILLIS: f64 = 1000.;
pub const MINS2MILLIS: f64 = 60. * SECS2MILLIS;
pub const HOURS2MILLIS: f64 = 60. * MINS2MILLIS;
pub const DAYS2MILLIS: f64 = 24. * HOURS2MILLIS;

pub const MILLIS2SECONDS: f64 = 0.001;
pub const MILLIS2MINUTES: f64 = MILLIS2SECONDS / 60.;
pub const MILLIS2HOURS: f64 = MILLIS2MINUTES / 60.;
pub const MILLIS2DAYS: f64 = MILLIS2HOURS / 24.;
pub const MILLIS2SECS: f64 = 0.001;

pub const SECS2MINS: f64 = 1. / 60.;
pub const HOURS2SECS: f64 = 1. / 60.;
pub const MINS2HOURS: f64 = 1. / 60.;

// 12pm 1st Jan 2000
pub const FIRST_JAN_2000_MILLIS: i64 = 946684800000;
pub const SYDNEY_28TH_APRIL_2023: i64 = 1682647200000;

#[derive(Debug, Clone, Copy, Deref, PartialEq, DerefMut)]
pub struct Y2000Day(pub f64);

impl Y2000Day {
	pub const FIRST_JAN_2000: Y2000Day = Y2000Day(1.);
	pub fn new(year: i32, month: u8, day: u8) -> Self {
		let julian = Date::from_calendar_date(year, u8_to_month(month), day)
			.unwrap()
			.to_julian_day() as f64;
		Y2000Day(julian - JULIAN_Y2000)
	}

	pub fn from_unix_ms(unix_ms: i64) -> Self {
		let offset_date =
			OffsetDateTime::UNIX_EPOCH + time::Duration::milliseconds(unix_ms);
		let julian = offset_date.to_julian_day_with_time();
		Y2000Day(julian - JULIAN_Y2000) //add1 because JULIAN_Y2000 is 0/Jan/2000?
	}

	pub fn add_utc_time(self, hour: u64, minute: u64, second: u64) -> Self {
		self.add_duration(Duration::from_hms(hour, minute, second))
	}
	pub fn add_hours(self, hours: f64) -> Self {
		self.add_duration(Duration::from_millis((hours * HOURS2MILLIS) as u64))
	}
	pub fn add_duration(self, duration: Duration) -> Self {
		Y2000Day(self.0 + duration.as_secs_f64() / SECS_IN_DAY)
	}

	
	pub fn days_since_j2000(&self) -> f64 { **self - 1.5 }

	pub fn centuries_since_j2000(&self) -> f64 {
		self.days_since_j2000() / 36525.0
	}

	pub fn now() -> Self {
		let now = OffsetDateTime::now_utc().to_julian_day_with_time();
		Y2000Day(now - JULIAN_Y2000)
	}

	pub fn utc_hour(&self) -> f64 { (**self % 1.0) * 24.0 }

	///obliquity of the ecliptic in degreees, decreasing
	#[rustfmt::skip]
	pub fn obl_ecl(&self) -> f64 {
		let t = self.centuries_since_j2000();
		23.4392911 
			- (46.8150 * t
			- 0.00059 * t * t 
			+ 0.001813 * t * t * t) 
			/ 3600.0
	}
	// pub fn obl_ecl_simple(&self) -> f64 { 23.43656 - 3.563E-7 * **self }

	/// GMST0 = Greenwich Mean Sidereal Time at Midnight
	pub fn gmst(&self) -> f64 {
		let l = OrbitalElements::get_l(*self, &ecliptic_positions::SUN);
		wrap_deg(l * DEG2HOURS + 12.)
	}

	// Local Mean Siderial Time
	pub fn lmst(&self, longitude: f64) -> f64 {
		self.gmst() + self.utc_hour() + longitude * DEG2HOURS
	}

	// Mean Siderial Time
	pub fn hour_angle(&self,longitude: f64,right_ascention:f64)->f64{
		wrap_hours(self.lmst(longitude) - right_ascention)
	}


	/// Greenwich Siderial Time
	pub fn gst(&self) -> f64 {
		let midnight = Y2000Day(self.floor());
		let t0 = midnight.centuries_since_j2000();
		let tut = (**self - *midnight) * 24.0;
		let sg = (6.6974 + 2400.0513 * t0) + (366.2422 / 365.2422) * tut;
		wrap_hours(sg)
	}

	// Local Sidereal Time
	pub fn lst(&self,longitude: f64)->f64{
		self.gst() + longitude * DEG2HOURS
		// let hour_angle = wrap_hours(lst - equatorial.right_ascention);
	}
	pub fn hour_angle_st(&self,longitude: f64,right_ascention:f64)->f64{
		wrap_hours(self.lst(longitude) - right_ascention)
	}
}

impl std::fmt::Display for Y2000Day {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// convert u8 to month, where 1 is january
pub fn u8_to_month(month: u8) -> Month {
	match month {
		1 => Month::January,
		2 => Month::February,
		3 => Month::March,
		4 => Month::April,
		5 => Month::May,
		6 => Month::June,
		7 => Month::July,
		8 => Month::August,
		9 => Month::September,
		10 => Month::October,
		11 => Month::November,
		12 => Month::December,
		_ => panic!("invalid month: {month}"),
	}
}

/// 2000-1-1 = 1
pub fn get_day_quickly(y: i32, m: u8, d: u8) -> f64 {
	let m = m as i32;
	let d = d as i32;
	//yes, integer division
	let d2000 = 367 * y
		- 7 * (y + (m + 9) / 12) / 4
		- 3 * ((y + (m - 9) / 7) / 100 + 1) / 4
		+ 275 * m / 9
		+ d - 730515;
	d2000 as f64
}
