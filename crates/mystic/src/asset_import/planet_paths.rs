use crate::astro::planets::Planet;
use std::collections::HashMap;
use strum::EnumCount;

pub fn get_planet_paths() -> HashMap<Planet, &'static str> {
	let mut map = HashMap::with_capacity(Planet::COUNT);
	for i in 0..Planet::COUNT {
		map.insert(i.try_into().unwrap(), PLANET_PATHS[i]);
	}
	map
}


pub const PLANET_PATHS: [&str; Planet::COUNT] = [
"https://upload.wikimedia.org/wikipedia/commons/a/aa/Sun_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/4/41/Moon_crescent_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/b/bb/Mercury_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/e/e3/Venus_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/0/0c/Earth_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/7/7a/Mars_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/d/d0/Jupiter_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/e/e1/Saturn_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/7/75/Uranus_monogram_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/c/c7/Neptune_symbol_%28fixed_width%29.svg",
"https://upload.wikimedia.org/wikipedia/commons/1/16/Pluto_symbol_%28large_orb%2C_fixed_width%29.svg",
];
