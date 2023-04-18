use super::super::*;
use super::*;

pub fn earth(day: Y2000Day) -> RectangluarCoords {
	let d = *day - 1.5;
	let t = d / 36525.0;
	let l0 = 280.46645 + (36000.76983 * t) + (0.0003032 * t * t);
	let m0 = 357.52910 + (35999.05030 * t)
		- (0.0001559 * t * t)
		- (0.00000048 * t * t * t);

	let c = (1.914600 - 0.004817 * t - 0.000014 * t * t) * sin_d(m0)
		+ (0.01993 - 0.000101 * t) * sin_d(2. * m0)
		+ 0.000290 * sin_d(3. * m0);
	let ls = l0 + c;
	let e = 0.016708617 - t * (0.000042037 + (0.0000001236 * t));
	let dist_au = (1.000001018 * (1. - e * e)) / (1. + e * cos_d(m0 + c));
	let x = -dist_au * cos_d(ls);
	let y = -dist_au * sin_d(ls);
	RectangluarCoords::new(x, y, 0.0)
}

pub const EARTH: OrbitalConstants = OrbitalConstants {
	//identical to sun with N flipped 180
	n_offset: 180.,
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
