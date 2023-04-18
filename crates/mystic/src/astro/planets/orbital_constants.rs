use super::*;
use std::collections::HashMap;
use strum::EnumCount;

pub struct OrbitalConstants {
	/// Longitude of asc. node
	pub N_offset: f64,
	pub N_scalar: f64,
	/// Inclination
	pub i_offset: f64,
	pub i_scalar: f64,
	/// Longitude of perihelion
	pub w_offset: f64,
	pub w_scalar: f64,
	/// mean distance, a.u.
	pub a_offset: f64,
	pub a_scalar: f64,
	/// eccentricity
	pub e_offset: f64,
	pub e_scalar: f64,
	/// mean anomaly
	pub M_offset: f64,
	pub M_scalar: f64,
}

pub fn get_orbital_constants() -> HashMap<Body, OrbitalConstants> {
	let mut map = HashMap::with_capacity(Body::COUNT);
	map.insert(Body::Sun, SUN);
	map.insert(Body::Mercury, MERCURY);
	map.insert(Body::Venus, VENUS);
	map.insert(Body::Earth, EARTH);
	map.insert(Body::Moon, MOON);
	map.insert(Body::Mars, MARS);
	map.insert(Body::Jupiter, JUPITER);
	map.insert(Body::Saturn, SATURN);
	map.insert(Body::Uranus, URANUS);
	map.insert(Body::Neptune, NEPTUNE);
	map
}

const SUN: OrbitalConstants = OrbitalConstants {
	N_offset: 0.,
	N_scalar: 0.,
	i_offset: 0.,
	i_scalar: 0.,
	w_offset: 282.9404,
	w_scalar: 4.70935E-5,
	a_offset: 1.000000,
	a_scalar: 0.,
	e_offset: 0.016709,
	e_scalar: -1.151E-9,
	M_offset: 356.0470,
	M_scalar: 0.9856002585,
};
const MOON: OrbitalConstants = OrbitalConstants {
	N_offset: 125.1228,
	N_scalar: -0.0529538083,
	i_offset: 5.1454,
	i_scalar: 0.,
	w_offset: 318.0634,
	w_scalar: 0.1643573223, //(Arg. of perigee)
	a_offset: 60.2666,      //earth radii
	a_scalar: 0.,
	e_offset: 0.054900,
	e_scalar: 0.,
	M_offset: 115.3654,
	M_scalar: 13.0649929509,
};
const MERCURY: OrbitalConstants = OrbitalConstants {
	N_offset: 48.3313,
	N_scalar: 3.24587E-5,
	i_offset: 7.0047,
	i_scalar: 5.00E-8,
	w_offset: 29.1241,
	w_scalar: 1.01444E-5,
	a_offset: 0.387098,
	a_scalar: 0.,
	e_offset: 0.205635,
	e_scalar: 5.59E-10,
	M_offset: 168.6562,
	M_scalar: 4.0923344368,
};

const VENUS: OrbitalConstants = OrbitalConstants {
	N_offset: 76.6799,
	N_scalar: 2.46590E-5,
	i_offset: 3.3946,
	i_scalar: 2.75E-8,
	w_offset: 54.8910,
	w_scalar: 1.38374E-5,
	a_offset: 0.723330,
	a_scalar: 0.,
	e_offset: 0.006773,
	e_scalar: -1.302E-9,
	M_offset: 48.0052,
	M_scalar: 1.6021302244,
};

const EARTH: OrbitalConstants = OrbitalConstants {
	//values copied from sun with N flipped 180
	N_offset: 180.,
	N_scalar: 0.,
	i_offset: 0.,
	i_scalar: 0.,
	w_offset: 282.9404,
	w_scalar: 4.70935E-5,
	a_offset: 1.,
	a_scalar: 0.,
	e_offset: 0.016709,
	e_scalar: -1.151E-9,
	M_offset: 356.0470,
	M_scalar: 0.9856002585,
};

const MARS: OrbitalConstants = OrbitalConstants {
	N_offset: 49.5574,
	N_scalar: 2.11081E-5,
	i_offset: 1.8497,
	i_scalar: -1.78E-8,
	w_offset: 286.5016,
	w_scalar: 2.92961E-5,
	a_offset: 1.523688,
	a_scalar: 0.,
	e_offset: 0.093405,
	e_scalar: 2.516E-9,
	M_offset: 18.6021,
	M_scalar: 0.5240207766,
};

const JUPITER: OrbitalConstants = OrbitalConstants {
	N_offset: 100.4542,
	N_scalar: 2.76854E-5,
	i_offset: 1.3030,
	i_scalar: -1.557E-7,
	w_offset: 273.8777,
	w_scalar: 1.64505E-5,
	a_offset: 5.20256,
	a_scalar: 0.,
	e_offset: 0.048498,
	e_scalar: 4.469E-9,
	M_offset: 19.8950,
	M_scalar: 0.0830853001,
};

const SATURN: OrbitalConstants = OrbitalConstants {
	N_offset: 113.6634,
	N_scalar: 2.38980E-5,
	i_offset: 2.4886,
	i_scalar: -1.081E-7,
	w_offset: 339.3939,
	w_scalar: 2.97661E-5,
	a_offset: 9.55475,
	a_scalar: 0.,
	e_offset: 0.055546,
	e_scalar: -9.499E-9,
	M_offset: 316.9670,
	M_scalar: 0.0334442282,
};

const URANUS: OrbitalConstants = OrbitalConstants {
	N_offset: 74.0005,
	N_scalar: 1.3978E-5,
	i_offset: 0.7733,
	i_scalar: 1.9E-8,
	w_offset: 96.6612,
	w_scalar: 3.0565E-5,
	a_offset: 19.18171,
	a_scalar: -1.55E-8, //Grand Uranus-Neptune term
	e_offset: 0.047318,
	e_scalar: 7.45E-9,
	M_offset: 142.5905,
	M_scalar: 0.011725806,
};

const NEPTUNE: OrbitalConstants = OrbitalConstants {
	N_offset: 131.7806,
	N_scalar: 3.0173E-5,
	i_offset: 1.7700,
	i_scalar: -2.55E-7,
	w_offset: 272.8461,
	w_scalar: -6.027E-6,
	a_offset: 30.05826,
	a_scalar: 3.313E-8, //Grand Uranus-Neptune term
	e_offset: 0.008606,
	e_scalar: 2.15E-9,
	M_offset: 260.2471,
	M_scalar: 0.005995147,
};
