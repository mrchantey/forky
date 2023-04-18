use super::super::*;
use super::*;

pub fn moon(day: Y2000Day) -> RectangluarCoords {
	let pos = OrbitalElements::position(&MOON, day);
	let pos = perturb(&pos, day);
	earth(day) + pos
}

fn perturb(pos: &RectangluarCoords, day: Y2000Day) -> RectangluarCoords {
	let ms = OrbitalElements::get_m(&SUN, day);
	let ws = OrbitalElements::get_w(&SUN, day);
	let ls = ms + ws;

	let mm = OrbitalElements::get_m(&MOON,day);
	let nm = OrbitalElements::get_n(&MOON,day);
	let wm = OrbitalElements::get_w(&MOON,day);
	let lm = mm + wm + nm;

	let d = lm - ls;
	let f = lm - nm;

	let mut ecliptic = pos.to_ecliptical();

	ecliptic.longitude += -1.274 * sin_d(mm - 2. * d) + 0.658 * sin_d(2. * d)
		- 0.186 * sin_d(ms)
		- 0.059 * sin_d(2. * mm - 2. * d)
		- 0.057 * sin_d(mm - 2. * d + ms)
		+ 0.053 * sin_d(mm + 2. * d)
		+ 0.046 * sin_d(2. * d - ms)
		+ 0.041 * sin_d(mm - ms)
		- 0.035 * sin_d(d)
		- 0.031 * sin_d(mm + ms)
		- 0.015 * sin_d(2. * f - 2. * d)
		+ 0.011 * sin_d(mm - 4. * d);

	ecliptic.latitude += -0.173 * sin_d(f - 2. * d)
		- 0.055 * sin_d(mm - f - 2. * d)
		- 0.046 * sin_d(mm + f - 2. * d)
		+ 0.033 * sin_d(f + 2. * d)
		+ 0.017 * sin_d(2. * mm + f);

	ecliptic.radius += (-0.58 * cos_d(mm - 2. * d) - 0.46 * cos_d(2. * d))
		/ EARTH_RADII_PER_AU;

	ecliptic.to_rectangular()
}

pub const MOON: OrbitalConstants = OrbitalConstants {
	n_offset: 125.1228,
	n_scalar: -0.0529538083,
	i_offset: 5.1454,
	i_scalar: 0.,
	w_offset: 318.0634,
	w_scalar: 0.1643573223,
	a_offset: 60.2666 / EARTH_RADII_PER_AU, //special case cos around earth
	a_scalar: 0.,
	e_offset: 0.054900,
	e_scalar: 0.,
	m_offset: 115.3654,
	m_scalar: 13.0649929509,
};
