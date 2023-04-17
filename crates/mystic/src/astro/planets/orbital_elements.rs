use super::*;

#[derive(Debug, Copy, Clone)]
pub struct OrbitalElements {
	///PRIMARY ELEMENTS

	///longitude of the ascending node, point where the body passes from negative to positive latitude on the ecliptic
	pub N: f64,
	///inclination to the ecliptic (plane of the Earth's orbit)
	pub i: f64,
	///argument of perihelion
	pub w: f64,
	///semi-major axis, or mean distance from Sun
	pub a: f64,
	///eccentricity (0=circle, 0-1=ellipse, 1=parabola)
	pub e: f64,
	///mean anomaly (0 at perihelion; increases uniformly with time)
	pub M: f64,

	//SECONDARY ELEMENTS
	///M + w1  = mean longitude
	pub L: f64,
	///true anomaly (angle between position and perihelion)
	pub v: f64,
	///eccentric anomaly
	pub E: f64,

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
		//primary
		let N = wrapDeg(constants.N_offset + constants.N_scalar * *day);
		let i = wrapDeg(constants.i_offset + constants.i_scalar * *day);
		let w = wrapDeg(constants.w_offset + constants.w_scalar * *day);
		let a = wrapDeg(constants.a_offset + constants.a_scalar * *day);
		let e = wrapDeg(constants.e_offset + constants.e_scalar * *day);
		let M = wrapDeg(constants.M_offset + constants.M_scalar * *day);

		//secondary

		let L = wrapDeg(N + w + M);
		let mut E0 = M + rad2deg * e * sin_d(M) * (1. + e * cos_d(M));
		let mut E1 = 0.;
		loop {
			let temp = E0;
			E0 = E1;
			E1 = temp
				- (temp - rad2deg * e * sin_d(temp) - M)
					/ (1. - e * cos_d(temp));
			if f64::abs(E1 - E0) <= 0.005 {
				break;
			}
		}
		let E = E1;
		let x = a * (cos_d(E) - e);
		let y = a * f64::sqrt(1. - e * e) * sin_d(E);
		let r = f64::sqrt(x * x + y * y);
		let v = wrapDeg(atan2_d(y, x)); //is wrapDeg nessecary? ie is negative bad?

		OrbitalElements {
			N,
			i,
			w,
			a,
			e,
			M,
			L,
			v,
			E,
			r,
		}
	}
}
