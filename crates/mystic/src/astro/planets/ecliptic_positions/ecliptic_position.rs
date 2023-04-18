use super::super::*;
use super::*;


pub fn ecliptic_position(day: Y2000Day, body: Body) -> RectangluarCoords {
	match body {
		Body::Sun => sun(),
		Body::Moon => moon(day),

		Body::Mercury => mercury(day),
		Body::Venus => venus(day),
		Body::Mars => mars(day),
		Body::Earth => earth(day),
		Body::Jupiter => jupiter(day),
		Body::Saturn => saturn(day),
		Body::Uranus => uranus(day),
		Body::Neptune => neptune(day),
		Body::Pluto => pluto(day),
	}
}

pub fn ecliptic_position_geo(day: Y2000Day, body: Body) -> RectangluarCoords {
	ecliptic_position_geo_with_earth(day, body, earth(day))
}
pub fn ecliptic_position_geo_with_earth(
	day: Y2000Day,
	body: Body,
	pos_earth: RectangluarCoords,
) -> RectangluarCoords {
	let mut pos = ecliptic_position(day, body);
	pos.x -= pos_earth.x;
	pos.y -= pos_earth.y;
	pos.z -= pos_earth.z;
	pos
}
