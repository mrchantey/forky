use crate::astro::chart::Sign;
use std::collections::HashMap;
use strum::EnumCount;


pub fn get_sign_paths() -> HashMap<Sign, &'static str> {
	let mut map = HashMap::with_capacity(Sign::COUNT);
	for i in 0..Sign::COUNT {
		map.insert(i.try_into().unwrap(), SIGN_PATHS[i]);
	}
	map
}

pub const SIGN_PATHS: [&str; Sign::COUNT] = [
"https://upload.wikimedia.org/wikipedia/commons/0/00/Aries_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/0/0b/Taurus_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/0/0c/Gemini_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/e/ec/Cancer_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/2/2c/Leo_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/a/a8/Virgo_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/0/07/Libra_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/7/7c/Scorpius_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/5/52/Sagittarius_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/a/a9/Capricornus_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/f/fd/Aquarius_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/2/21/Pisces_symbol_%28fixed_width%29.svg",
];
