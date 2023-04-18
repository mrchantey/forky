use super::*;
use crate::astro::planets::*;




pub fn sun_geo(day: Y2000Day) -> RectangluarCoords {
	let earth_helio = earth_helio(day);
	RectangluarCoords::new(-earth_helio.x, -earth_helio.y, 0.)
}

pub fn sun_helio() -> RectangluarCoords { RectangluarCoords::new(0., 0., 0.) }
