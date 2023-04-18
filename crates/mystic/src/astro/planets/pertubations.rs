use super::*;
use std::collections::HashMap;

pub fn apply_pertubations(bodies: &mut HashMap<Body, OrbitalBody>) {
	let el_sun = bodies.get(&Body::Sun).unwrap().el;
	let el_saturn = bodies.get(&Body::Saturn).unwrap().el;
	let el_jupiter = bodies.get(&Body::Jupiter).unwrap().el;

	moon(bodies.get_mut(&Body::Moon).unwrap(), &el_sun);
	jupiter(bodies.get_mut(&Body::Jupiter).unwrap(), &el_saturn);
	saturn(bodies.get_mut(&Body::Saturn).unwrap(), &el_jupiter);
	uranus(
		bodies.get_mut(&Body::Uranus).unwrap(),
		&el_jupiter,
		&el_saturn,
	);
}

fn moon(moon: &mut OrbitalBody, sun: &OrbitalElements) {
	let M = moon.el.M;
	let Ms = sun.M;
	let F = moon.el.L - moon.el.N;
	let D = moon.el.L - sun.L;
	moon.ecliptic.longitude += -1.274 * sin_d(M - 2. * D)
		+ 0.658 * sin_d(2. * D)
		- 0.186 * sin_d(Ms)
		- 0.059 * sin_d(2. * M - 2. * D)
		- 0.057 * sin_d(M - 2. * D + Ms)
		+ 0.053 * sin_d(M + 2. * D)
		+ 0.046 * sin_d(2. * D - Ms)
		+ 0.041 * sin_d(M - Ms)
		- 0.035 * sin_d(D)
		- 0.031 * sin_d(M + Ms)
		- 0.015 * sin_d(2. * F - 2. * D)
		+ 0.011 * sin_d(M - 4. * D);
	moon.ecliptic.latitude += -0.173 * sin_d(F - 2. * D)
		- 0.055 * sin_d(M - F - 2. * D)
		- 0.046 * sin_d(M + F - 2. * D)
		+ 0.033 * sin_d(F + 2. * D)
		+ 0.017 * sin_d(2. * M + F);
	moon.ecliptic.radius += -0.58 * cos_d(M - 2. * D) - 0.46 * cos_d(2. * D);

	//FOR TESTING ONLY
	// moon.ecliptic.radius = 1
	// moon.ecliptic.longitude = 306.9484
	moon.ecliptic_rect = moon.ecliptic.to_rectangular();
}

pub fn jupiter(jupiter: &mut OrbitalBody, saturn: &OrbitalElements) {
	let Mj = jupiter.el.M;
	let Ms = saturn.M;
	jupiter.ecliptic.longitude += -0.332 * sin_d(2. * Mj - 5. * Ms - 67.6)
		- 0.056 * sin_d(2. * Mj - 2. * Ms + 21.)
		+ 0.042 * sin_d(3. * Mj - 5. * Ms + 21.)
		- 0.036 * sin_d(Mj - 2. * Ms)
		+ 0.022 * cos_d(Mj - Ms)
		+ 0.023 * sin_d(2. * Mj - 3. * Ms + 52.)
		- 0.016 * sin_d(Mj - 5. * Ms - 69.);
	jupiter.ecliptic_rect = jupiter.ecliptic.to_rectangular();
}

pub fn saturn(saturn: &mut OrbitalBody, jupiter: &OrbitalElements) {
	let Mj = jupiter.M;
	let Ms = saturn.el.M;
	saturn.ecliptic.longitude += 0.812 * sin_d(2. * Mj - 5. * Ms - 67.6)
		- 0.229 * cos_d(2. * Mj - 4. * Ms - 2.)
		+ 0.119 * sin_d(Mj - 2. * Ms - 3.)
		+ 0.046 * sin_d(2. * Mj - 6. * Ms - 69.)
		+ 0.014 * sin_d(Mj - 3. * Ms + 32.);
	saturn.ecliptic.latitude += -0.020 * cos_d(2. * Mj - 4. * Ms - 2.)
		+ 0.018 * sin_d(2. * Mj - 6. * Ms - 49.);
	saturn.ecliptic_rect = saturn.ecliptic.to_rectangular();
}

pub fn uranus(
	uranus: &mut OrbitalBody,
	jupiter: &OrbitalElements,
	saturn: &OrbitalElements,
) {
	let Mj = jupiter.M;
	let Ms = saturn.M;
	let Mu = uranus.el.M;
	uranus.ecliptic.longitude += 0.040 * sin_d(Ms - 2. * Mu + 6.)
		+ 0.035 * sin_d(Ms - 3. * Mu + 33.)
		- 0.015 * sin_d(Mj - Mu + 20.);
	uranus.ecliptic_rect = uranus.ecliptic.to_rectangular();
}
