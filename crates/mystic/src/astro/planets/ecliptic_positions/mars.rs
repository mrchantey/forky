use super::super::*;
use super::*;

pub fn mars(day: Y2000Day) -> RectCoords {
	OrbitalElements::position(&MARS, day)
}

pub const MARS: OrbitalConstants = OrbitalConstants {
	n_offset: 49.5574,
	n_scalar: 2.11081E-5,
	i_offset: 1.8497,
	i_scalar: -1.78E-8,
	w_offset: 286.5016,
	w_scalar: 2.92961E-5,
	a_offset: 1.523688,
	a_scalar: 0.,
	e_offset: 0.093405,
	e_scalar: 2.516E-9,
	m_offset: 18.6021,
	m_scalar: 0.5240207766,
};
