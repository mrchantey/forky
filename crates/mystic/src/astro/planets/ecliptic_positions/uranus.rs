use super::super::*;
use super::*;

pub fn uranus(day: Y2000Day) -> RectangluarCoords {
	let pos = OrbitalElements::position(&URANUS_FROM_JPL, day);
	perturb(&pos, day)
}

fn perturb(pos: &RectangluarCoords, day: Y2000Day) -> RectangluarCoords {
	let mj = OrbitalElements::get_m(&JUPITER_FROM_JPL, day);
	let ms = OrbitalElements::get_m(&SATURN, day);
	let mu = OrbitalElements::get_m(&URANUS_FROM_JPL, day);
	let mut ecliptic = pos.to_ecliptical();
	ecliptic.longitude += 0.040 * sin_d(ms - 2. * mu + 6.)
		+ 0.035 * sin_d(ms - 3. * mu + 33.)
		- 0.015 * sin_d(mj - mu + 20.);

	ecliptic.to_rectangular()
}

pub const URANUS_FROM_JPL: OrbitalConstants = OrbitalConstants {
	n_offset: 74.016923288485,
	n_scalar: 1.161009993155373e-6,
	i_offset: 0.7726379297696099,
	i_scalar: -6.651307323750856e-8,
	w_offset: 96.93733625369939,
	w_scalar: 1.0010867077344285e-5,
	a_offset: 19.189164720565095,
	a_scalar: -5.371006160164271e-8,
	e_offset: 0.047257441805749485,
	e_scalar: -1.203832991101985e-9,
	m_offset: 142.26624816995735,
	m_scalar: 0.011720026695140315,
};

pub const URANUS: OrbitalConstants = OrbitalConstants {
	n_offset: 74.0005,
	n_scalar: 1.3978E-5,
	i_offset: 0.7733,
	i_scalar: 1.9E-8,
	w_offset: 96.6612,
	w_scalar: 3.0565E-5,
	a_offset: 19.18171,
	a_scalar: -1.55E-8,
	e_offset: 0.047318,
	e_scalar: 7.45E-9,
	m_offset: 142.5905,
	m_scalar: 0.011725806,
};

pub const URANUS_JPL: JPLConstants = JPLConstants::new(
	19.18916464,
	-0.00196176,
	0.04725744,
	-0.00004397,
	0.77263783,
	-0.00242939,
	313.23810451,
	428.48202785,
	170.95427630,
	0.40805281,
	74.01692503,
	0.04240589,
);
