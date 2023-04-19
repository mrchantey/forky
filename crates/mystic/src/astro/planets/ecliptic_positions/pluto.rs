use super::super::*;

pub fn pluto(day: Y2000Day) -> RectCoords {
	let s = 50.03 + (0.033459652 * *day);
	let p = 238.95 + (0.003968789 * *day);

	let lonecl = 238.9508 + (0.00400703 * *day) - 19.799 * sin_d(p)
		+ 19.848 * cos_d(p)
		+ 0.897 * sin_d(2. * p)
		- 4.956 * cos_d(2. * p)
		+ 0.610 * sin_d(3. * p)
		+ 1.211 * cos_d(3. * p)
		- 0.341 * sin_d(4. * p)
		- 0.190 * cos_d(4. * p)
		+ 0.128 * sin_d(5. * p)
		- 0.034 * cos_d(5. * p)
		- 0.038 * sin_d(6. * p)
		+ 0.031 * cos_d(6. * p)
		+ 0.020 * sin_d(s - p)
		- 0.010 * cos_d(s - p);

	let latecl = -3.9082 - 5.453 * sin_d(p) - 14.975 * cos_d(p)
		+ 3.527 * sin_d(2. * p)
		+ 1.673 * cos_d(2. * p)
		- 1.051 * sin_d(3. * p)
		+ 0.328 * cos_d(3. * p)
		+ 0.179 * sin_d(4. * p)
		- 0.292 * cos_d(4. * p)
		+ 0.019 * sin_d(5. * p)
		+ 0.100 * cos_d(5. * p)
		- 0.031 * sin_d(6. * p)
		- 0.026 * cos_d(6. * p)
		+ 0.011 * cos_d(s - p);

	let r = 40.72 + 6.68 * sin_d(p) + 6.90 * cos_d(p)
		- 1.18 * sin_d(2. * p)
		- 0.03 * cos_d(2. * p)
		+ 0.15 * sin_d(3. * p)
		- 0.14 * cos_d(3. * p);

	let coslon = cos_d(lonecl);
	let sinlon = sin_d(lonecl);
	let coslat = cos_d(latecl);
	let sinlat = sin_d(latecl);

	let xp = r * coslon * coslat;
	let yp = r * sinlon * coslat;
	let zp = r * sinlat;

	RectCoords::new(xp, yp, zp)
}
