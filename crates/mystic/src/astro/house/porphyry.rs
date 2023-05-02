use crate::astro::{
	chart::ZodiacPosition,
	planets::{wrap_deg, EclipticCoords, GeographicCoords, Y2000Day, DEG2RAD},
};
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
/// Porphyry house system, even spacing between I, IV, VII, and X.
pub struct PorphyryHouse(pub [ZodiacPosition; 12]);

impl PorphyryHouse {
	pub fn new(day: Y2000Day, position: &GeographicCoords) -> Self {
		let mut houses: [ZodiacPosition; 12] = Default::default();

		let (i, iv, vii, x) = get_i_iv_vii_x(day, position);

		let q1 = unsigned_angle_between(i, iv) / 3.;
		let ii = i + 1. * q1;
		let iii = i + 2. * q1;
		let viii = vii + 1. * q1;
		let ix = vii + 2. * q1;
		let q2 = unsigned_angle_between(iv, vii) / 3.;
		let v = iv + 1. * q2;
		let vi = iv + 2. * q2;
		let xi = x + 1. * q2;
		let xii = x + 2. * q2;

		houses[0] = ZodiacPosition::from_angle(i * DEG2RAD);
		houses[1] = ZodiacPosition::from_angle(ii * DEG2RAD);
		houses[2] = ZodiacPosition::from_angle(iii * DEG2RAD);
		houses[3] = ZodiacPosition::from_angle(iv * DEG2RAD);
		houses[4] = ZodiacPosition::from_angle(v * DEG2RAD);
		houses[5] = ZodiacPosition::from_angle(vi * DEG2RAD);
		houses[6] = ZodiacPosition::from_angle(vii * DEG2RAD);
		houses[7] = ZodiacPosition::from_angle(viii * DEG2RAD);
		houses[8] = ZodiacPosition::from_angle(ix * DEG2RAD);
		houses[9] = ZodiacPosition::from_angle(x * DEG2RAD);
		houses[10] = ZodiacPosition::from_angle(xi * DEG2RAD);
		houses[11] = ZodiacPosition::from_angle(xii * DEG2RAD);


		// let delta = 1. / 12.;
		// for i in 0..12 {
		// 	let day = Y2000Day(*day + delta * i as f64);
		// 	// println!("day: {}", *day);
		// 	houses[i] =
		// 		EclipticCoords::eastern_horizon_intersect(day, &position)
		// 			.into();
		// }
		Self(houses)
	}
}


pub fn get_i_iv_vii_x(
	day: Y2000Day,
	position: &GeographicCoords,
) -> (f64, f64, f64, f64) {
	let i = EclipticCoords::eastern_horizon_intersect(day, &position).longitude;
	let vii = wrap_deg(i + 180.);
	let x = EclipticCoords::meridian_intersect(day, &position).longitude;
	let iv = wrap_deg(x + 180.);
	(i, iv, vii, x)
}

fn unsigned_mod(a: f64, n: f64) -> f64 { a - f64::floor(a / n) * n }
//https://stackoverflow.com/questions/1878907/how-can-i-find-the-difference-between-two-angles
fn angle_between(a: f64, b: f64) -> f64 {
	unsigned_mod(a - b + 180., 360.) - 180.
}
fn unsigned_angle_between(a: f64, b: f64) -> f64 {
	f64::abs(angle_between(a, b))
}
