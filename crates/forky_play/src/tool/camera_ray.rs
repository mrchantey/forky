use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CameraRay {
	pub ray: Ray,
	pub entity: Option<Entity>,
	pub origin_intersect: Option<RayIntersect>,
	pub entity_intersect: Option<RayIntersect>,
}

pub struct RayIntersect {
	pub ray: Ray,
	pub position: Vec3,
	pub delta: Vec3,
}

impl RayIntersect {
	pub fn new(ray: Ray, dist: f32, last: &Option<RayIntersect>) -> Self {
		let position = ray.origin + ray.direction * dist;
		let delta = if let Some(last) = last {
			position - last.position
		} else {
			Vec3::ZERO
		};
		RayIntersect {
			ray: ray.clone(),
			position,
			delta,
		}
	}
	pub fn from_plane(
		ray: Ray,
		origin: Vec3,
		up: Vec3,
		last: &Option<RayIntersect>,
	) -> Option<RayIntersect> {
		if let Some(dist) = ray.intersect_plane(origin, up) {
			Some(RayIntersect::new(ray, dist, &last))
		} else {
			None
		}
	}
}
// pub struct RayEntityIntersect {
// 	pub intersect: Option<RayIntersect>,
// 	pub entity: Entity,
// }
