use super::super::*;
use super::*;

pub fn mercury(day: Y2000Day) -> RectangluarCoords {
	OrbitalElements::position(&MERCURY, day)
}

pub const MERCURY: OrbitalConstants = OrbitalConstants {
	n_offset: 48.3313,
	n_scalar: 3.24587E-5,
	i_offset: 7.0047,
	i_scalar: 5.00E-8,
	w_offset: 29.1241,
	w_scalar: 1.01444E-5,
	a_offset: 0.387098,
	a_scalar: 0.,
	e_offset: 0.205635,
	e_scalar: 5.59E-10,
	m_offset: 168.6562,
	m_scalar: 4.0923344368,
};
