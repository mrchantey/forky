use super::super::*;
use super::*;

pub fn venus(day: Y2000Day) -> HelioCoords {
	OrbitalElements::position(&VENUS, day)
}

pub const VENUS: OrbitalConstants = OrbitalConstants {
	n_offset: 76.6799,
	n_scalar: 2.46590E-5,
	i_offset: 3.3946,
	i_scalar: 2.75E-8,
	w_offset: 54.8910,
	w_scalar: 1.38374E-5,
	a_offset: 0.723330,
	a_scalar: 0.,
	e_offset: 0.006773,
	e_scalar: -1.302E-9,
	m_offset: 48.0052,
	m_scalar: 1.6021302244,
};
