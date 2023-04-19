use super::super::*;
use super::*;

pub fn jupiter(day: Y2000Day) -> HelioCoords {
	let pos = OrbitalElements::position(day, &JUPITER_FROM_JPL);
	perturb(day, &pos)
}

fn perturb(day: Y2000Day, pos: &HelioCoords) -> HelioCoords {
	let mj = OrbitalElements::get_m(day, &JUPITER_FROM_JPL);
	//astronomy.js uses saturn, but i think it should be saturn_from_jpl
	let ms = OrbitalElements::get_m(day, &SATURN);
	let mut ecliptic = pos.to_spherical();

	ecliptic.longitude += -0.332 * sin_d(2. * mj - 5. * ms - 67.6)
		- 0.056 * sin_d(2. * mj - 2. * ms + 21.)
		+ 0.042 * sin_d(3. * mj - 5. * ms + 21.)
		- 0.036 * sin_d(mj - 2. * ms)
		+ 0.022 * cos_d(mj - ms)
		+ 0.023 * sin_d(2. * mj - 3. * ms + 52.)
		- 0.016 * sin_d(mj - 5. * ms - 69.);

	ecliptic.to_rectangular().into()
}

pub const JUPITER_FROM_JPL: OrbitalConstants = OrbitalConstants {
	n_offset: 100.47390068379632,
	n_scalar: 5.604135797399042e-6,
	i_offset: 1.304396874552772,
	i_scalar: 5.0298151950718684e-8,
	w_offset: 274.25457041820863,
	w_scalar: 2.1452758384668002e-7,
	a_offset: 5.202887004766735,
	a_scalar: -3.17782340862423e-9,
	e_offset: 0.04838624544271047,
	e_scalar: -3.628473648186174e-9,
	m_offset: 19.54333917687597,
	m_scalar: 0.0830810020826831,
};
pub const JUPITER: OrbitalConstants = OrbitalConstants {
	n_offset: 100.4542,
	n_scalar: 2.76854E-5,
	i_offset: 1.3030,
	i_scalar: -1.557E-7,
	w_offset: 273.8777,
	w_scalar: 1.64505E-5,
	a_offset: 5.20256,
	a_scalar: 0.,
	e_offset: 0.048498,
	e_scalar: 4.469E-9,
	m_offset: 19.8950,
	m_scalar: 0.0830853001,
};


pub const JUPITER_JPL: JPLConstants = JPLConstants::new(
	5.20288700,
	-0.00011607,
	0.04838624,
	-0.00013253,
	1.30439695,
	0.00183714,
	34.39644051,
	3034.74612775,
	14.72847983,
	0.21252668,
	100.47390909,
	0.20469106,
);
