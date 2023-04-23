use super::super::planets::GeographicCoords;
use crate::astro::planets::Y2000Day;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::io;


#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Date {
	pub day: u8,
	pub month: u8,
	pub year: i32,
}
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarthMoment {
	pub date: Date,
	pub time: Time,
	pub location: GeographicCoords,
}

impl Into<Y2000Day> for EarthMoment {
	fn into(self) -> Y2000Day {
		Y2000Day::new(self.date.year, self.date.month, self.date.day)
			.add_utc_time(
				self.time.hour as u64,
				self.time.minute as u64,
				self.time.second as u64,
			)
	}
}

impl EarthMoment {
	pub fn from_std() -> Result<EarthMoment> {
		println!(
			"Be sure to use UTC times and dates: https://savvytime.com/converter/utc"
		);

		println!("Enter your UTC birthday in the format: DD/MM/YYYY");
		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		let date: Vec<&str> = input.split('/').collect();
		let date = Date {
			day: date[0].trim().parse()?,
			month: date[1].trim().parse()?,
			year: date[2].trim().parse()?,
		};
		println!(
			"Enter your UTC birth time in the 24hr format: hh:mm, ie 21:30"
		);
		input.clear();
		io::stdin().read_line(&mut input)?;
		let time: Vec<&str> = input.split(':').collect();
		let time = Time {
			hour: time[0].trim().parse()?,
			minute: time[1].trim().parse()?,
			second: 0,
		};
		println!(
			"Enter your birth location in the format lat,long ie -20.382, 150.202.\nhttps://www.latlong.net/"
		);
		input.clear();
		io::stdin().read_line(&mut input)?;
		let location: Vec<&str> = input.split(',').collect();
		let location = GeographicCoords {
			latitude: location[0].trim().parse()?,
			longitude: location[1].trim().parse()?,
			altitude: 0.,
		};

		Ok(EarthMoment {
			date,
			time,
			location,
		})
	}
}
