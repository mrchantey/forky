use super::*;
use crate::astro::planets::*;

pub fn sun() -> HelioCoords { HelioCoords::new(0., 0., 0.) }

pub const SUN: OrbitalConstants = OrbitalConstants {
	//identical to earth with N flipped 180
	n_offset: 0.,
	n_scalar: 0.,
	i_offset: 0.,
	i_scalar: 0.,
	w_offset: 282.9404,
	w_scalar: 4.70935E-5,
	a_offset: 1.,
	a_scalar: 0.,
	e_offset: 0.016709,
	e_scalar: -1.151E-9,
	m_offset: 356.0470,
	m_scalar: 0.9856002585,
};
