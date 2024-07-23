use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraRay {
	pub ray: Ray3d,
	pub entity: Option<Entity>,
	pub origin_intersect: Option<RayIntersect>,
	pub entity_intersect: Option<RayIntersect>,
}

impl Default for CameraRay {
	fn default() -> Self {
		Self {
			ray: Ray3d::new(Vec3::ZERO, -Vec3::Z),
			entity: None,
			origin_intersect: None,
			entity_intersect: None,
		}
	}
}


pub struct RayIntersect {
	pub ray: Ray3d,
	pub position: Vec3,
	pub delta: Vec3,
}


impl RayIntersect {
	pub fn new(ray: Ray3d, dist: f32, last: &Option<RayIntersect>) -> Self {
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
		ray: Ray3d,
		origin: Vec3,
		up: Vec3,
		last: &Option<RayIntersect>,
	) -> Option<RayIntersect> {
		if let Some(dist) = ray.intersect_plane(origin, Plane3d::new(up)) {
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
