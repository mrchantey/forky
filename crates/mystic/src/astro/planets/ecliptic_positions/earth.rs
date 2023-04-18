use crate::astro::planets::*;



#[rustfmt::skip]
pub fn earth_geo()->RectangluarCoords{
	RectangluarCoords::new(0.,0.,0.)
}

pub fn earth_helio(day: Y2000Day) -> RectangluarCoords {
	let d = *day - 1.5;
	let T = d / 36525.0;
	let L0 = 280.46645 + (36000.76983 * T) + (0.0003032 * T * T);
	let M0 = 357.52910 + (35999.05030 * T)
		- (0.0001559 * T * T)
		- (0.00000048 * T * T * T);

	let C = (1.914600 - 0.004817 * T - 0.000014 * T * T) * sin_d(M0)
		+ (0.01993 - 0.000101 * T) * sin_d(2. * M0)
		+ 0.000290 * sin_d(3. * M0);
	let LS = L0 + C;
	let e = 0.016708617 - T * (0.000042037 + (0.0000001236 * T));
	let distanceInAU = (1.000001018 * (1. - e * e)) / (1. + e * cos_d(M0 + C));
	let x = -distanceInAU * cos_d(LS);
	let y = -distanceInAU * sin_d(LS);
	return RectangluarCoords::new(x, y, 0.0);
}
