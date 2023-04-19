use super::super::*;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct OrbitalElements {
	///PRIMARY ELEMENTS

	///longitude of the ascending node, point where the body passes from negative to positive latitude on the ecliptic
	pub n: f64,
	///inclination to the ecliptic (plane of the Earth's orbit)
	pub i: f64,
	///argument of perihelion
	pub w: f64,
	///semi-major axis, or mean distance from Sun
	pub a: f64,
	///eccentricity (0=circle, 0-1=ellipse, 1=parabola)
	pub e: f64,
	///mean anomaly (0 at perihelion; increases uniformly with time)
	pub m: f64,

	//SECONDARY ELEMENTS
	///M + w1  = mean longitude
	pub l: f64,
	///true anomaly (angle between position and perihelion)
	pub v: f64,
	///eccentric anomaly
	pub ea: f64,

	pub r: f64,
	// ///N + w = longitude of perihelion
	// w1:f64,
	// ///a*(1-e) = perihelion distance
	// q:f64,
	// ///a*(1+e) = aphelion distance
	// Q:f64,
	// ///a ^ 1.5 = orbital period (years if a is in AU, astronomical units)
	// P:f64,
	// ///Epoch_of_M - (M(deg)/360_deg) / P  = time of perihelion
	// T:f64,
}

impl OrbitalElements {
	pub fn new(constants: &OrbitalConstants, day: Y2000Day) -> Self {
		let a = constants.a_offset + (*day * constants.a_scalar);
		let e = constants.e_offset + (*day * constants.e_scalar);
		let m = constants.m_offset + (*day * constants.m_scalar);
		let n = constants.n_offset + (*day * constants.n_scalar);
		let w = constants.w_offset + (*day * constants.w_scalar);
		let i = constants.i_offset + (*day * constants.i_scalar);
		let l = n + w + m;
		let ea = eccentric_anomaly(e, m);

		let xv = a * (cos_d(ea) - e);
		let yv = a * (f64::sqrt(1.0 - e * e) * sin_d(ea));

		let v = atan2_d(yv, xv);
		let r = f64::sqrt(xv * xv + yv * yv);
		Self {
			n,
			i,
			w,
			a,
			e,
			m,
			l,
			v,
			ea,
			r,
		}
	}

	pub fn position(constants: &OrbitalConstants, day: Y2000Day) -> RectCoords {
		let OrbitalElements { n, i, w, v, r, .. } =
			OrbitalElements::new(constants, day);

		let cos_n = cos_d(n);
		let sin_n = sin_d(n);
		let cos_i = cos_d(i);
		let sin_i = sin_d(i);
		let cos_vw = cos_d(v + w);
		let sin_vw = sin_d(v + w);

		let x = r * (cos_n * cos_vw - sin_n * sin_vw * cos_i);
		let y = r * (sin_n * cos_vw + cos_n * sin_vw * cos_i);
		let z = r * sin_vw * sin_i;

		RectCoords::new(x, y, z)
	}
	///mean anomoly
	pub fn get_m(constants: &OrbitalConstants, day: Y2000Day) -> f64 {
		constants.m_offset + (*day * constants.m_scalar)
	}
	pub fn get_n(constants: &OrbitalConstants, day: Y2000Day) -> f64 {
		constants.n_offset + (*day * constants.n_scalar)
	}
	///argument of perihelion
	pub fn get_w(constants: &OrbitalConstants, day: Y2000Day) -> f64 {
		constants.w_offset + (*day * constants.w_scalar)
	}

	///mean longitude
	pub fn get_l(constants: &OrbitalConstants, day: Y2000Day) -> f64 {
		let m = Self::get_m(constants, day);
		let w = Self::get_w(constants, day);
		let n = Self::get_n(constants, day);
		n + w + m
	}
}



//increase to speed up calculations, ie 0.0001
const MAX_ERROR: f64 = 1.0e-8;

fn eccentric_anomaly(e: f64, m: f64) -> f64 {
	let mut e2 = m + (e * sin_d(m) * (1.0 + (e * cos_d(m))));
	loop {
		let f =
			e2 - (e2 - (RAD2DEG * e * sin_d(e2)) - m) / (1. - e * cos_d(e2));
		let error = f64::abs(f - e2);
		e2 = f;
		if error < MAX_ERROR {
			break;
		}
	}
	return e2;
}
