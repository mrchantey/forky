use super::super::*;
use super::*;

pub fn neptune(day: Y2000Day) -> HelioCoords {
	OrbitalElements::position(day, &NEPTUNE)
}

pub const NEPTUNE: OrbitalConstants = OrbitalConstants {
	n_offset: 131.7806,
	n_scalar: 3.0173E-5,
	i_offset: 1.7700,
	i_scalar: -2.55E-7,
	w_offset: 272.8461,
	w_scalar: -6.027E-6,
	a_offset: 30.05826,
	a_scalar: 3.313E-8,
	e_offset: 0.008606,
	e_scalar: 2.15E-9,
	m_offset: 260.2471,
	m_scalar: 0.005995147,
};
