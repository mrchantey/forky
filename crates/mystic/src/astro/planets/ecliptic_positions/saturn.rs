use super::super::*;
use super::*;

pub fn saturn(day: Y2000Day) -> RectangluarCoords {
	let el = OrbitalElements::new(&SATURN, day);
	perturb(&el, day)
}

fn perturb(el: &OrbitalElements, day: Y2000Day) -> RectangluarCoords {
	let mj = OrbitalElements::get_m(&JUPITER_FROM_JPL, day);
	let ms = el.m;

	let mut ecliptic = el.pos.to_ecliptical();
	ecliptic.longitude += 0.812 * sin_d(2. * mj - 5. * ms - 67.6)
		- 0.229 * cos_d(2. * mj - 4. * ms - 2.)
		+ 0.119 * sin_d(mj - 2. * ms - 3.)
		+ 0.046 * sin_d(2. * mj - 6. * ms - 69.)
		+ 0.014 * sin_d(mj - 3. * ms + 32.);
	ecliptic.latitude += -0.020 * cos_d(2. * mj - 4. * ms - 2.)
		+ 0.018 * sin_d(2. * mj - 6. * ms - 49.);
	ecliptic.to_rectangular()
}

pub const SATURN_FROM_JPL: OrbitalConstants = OrbitalConstants {
	n_offset: 113.66243633535687,
	n_scalar: -7.903571252566737e-6,
	i_offset: 2.485991790489117,
	i_scalar: 5.30072553045859e-8,
	w_offset: 338.93645918089203,
	w_scalar: -3.5672613278576307e-6,
	a_offset: 9.536675991359344,
	a_scalar: -3.4239561943874055e-8,
	e_offset: 0.053861810940862426,
	e_scalar: -1.3960574948665299e-8,
	m_offset: 317.3051436368719,
	m_scalar: 0.03348152208542095,
};
pub const SATURN: OrbitalConstants = OrbitalConstants {
	n_offset: 113.6634,
	n_scalar: 2.38980E-5,
	i_offset: 2.4886,
	i_scalar: -1.081E-7,
	w_offset: 339.3939,
	w_scalar: 2.97661E-5,
	a_offset: 9.55475,
	a_scalar: 0.,
	e_offset: 0.055546,
	e_scalar: -9.499E-9,
	m_offset: 316.9670,
	m_scalar: 0.0334442282,
};

pub const SATURN_JPL: JPLConstants = JPLConstants::new(
	9.53667594,
	-0.00125060,
	0.05386179,
	-0.00050991,
	2.48599187,
	0.00193609,
	49.95424423,
	1222.49362201,
	92.59887831,
	-0.41897216,
	113.66242448,
	-0.28867794,
);
