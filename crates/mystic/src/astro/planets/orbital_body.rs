use super::{ecliptic_positions::OrbitalElements, *};


#[derive(Debug, Copy, Clone)]
pub struct OrbitalBody {
	pub el: OrbitalElements,
	pub ecliptic: EclipticalCoords,
	pub ecliptic_rect: RectangluarCoords,
}


impl OrbitalBody {
	pub fn new(el: OrbitalElements) -> Self {
		let ecliptic_rect = RectangluarCoords {
			x: el.r
				* (cos_d(el.n) * cos_d(el.v + el.w)
					- sin_d(el.n) * sin_d(el.v + el.w) * cos_d(el.i)),
			y: el.r
				* (sin_d(el.n) * cos_d(el.v + el.w)
					+ cos_d(el.n) * sin_d(el.v + el.w) * cos_d(el.i)),
			z: el.r * sin_d(el.v + el.w) * sin_d(el.i),
		};

		let mut ecliptic = ecliptic_rect.to_ecliptical();
		ecliptic.longitude = wrap_deg(ecliptic.longitude); //nesecary?

		Self {
			el,
			ecliptic_rect,
			ecliptic,
		}
	}
}
