use bevy::prelude::*;
use extend::ext;

use crate::*;

#[ext]
pub impl SpatialBundle {
	fn from_xyz(x: f32, y: f32, z: f32) -> SpatialBundle {
		SpatialBundle {
			transform: Transform::from_position(Vec3::new(x, y, z)),
			..default()
		}
	}
	fn from_position(v: Vec3) -> SpatialBundle {
		SpatialBundle {
			transform: Transform::from_position(v),
			..default()
		}
	}
}
