use crate::astro::serde::EarthMoment;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_PATH: &str = "moment.json";

impl EarthMoment {
	pub fn to_disk(&self) -> Result<()> {
		let mut file = File::create(DEFAULT_PATH)?;
		let json = serde_json::to_string(self)?;
		file.write_all(json.as_bytes())?;
		Ok(())
	}

	pub fn from_disk() -> Result<Self> {
		let file = File::open(DEFAULT_PATH)?;
		let mut buf_reader = std::io::BufReader::new(file);
		let mut contents = String::new();
		buf_reader.read_to_string(&mut contents)?;
		let person = serde_json::from_str(&contents)?;
		Ok(person)
	}
}
