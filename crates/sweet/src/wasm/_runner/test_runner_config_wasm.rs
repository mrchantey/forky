use crate::*;
use forky_web::SearchParams;
use glob::Pattern;


pub const MATCHES_KEY: &str = "m";

impl TestRunnerConfig {
	pub fn from_search_params() -> Self {
		let matches = SearchParams::get_all(MATCHES_KEY)
			.iter()
			.map(|s| Pattern::new(&s).unwrap())
			.collect::<Vec<_>>();
		// if let Some(file) =  {
		// 	//todo error onn malformed pattern
		// 	matches.push(Pattern::new(&file).unwrap());
		// }
		Self {
			watch: false,
			parallel: false,
			matches,
		}
	}
}
