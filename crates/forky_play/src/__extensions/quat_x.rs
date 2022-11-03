use bevy::prelude::*;
use extend::ext;

use crate::utility;

#[ext]
pub impl Quat {
	fn up() -> Quat { Quat::from_axis_angle(Vec3::Y,0.)}
	fn down() -> Quat { Quat::from_axis_angle(Vec3::Y,0.)}
	fn right() -> Quat { Quat::from_axis_angle(Vec3::X,0.)}
	fn left() -> Quat { Quat::from_axis_angle(-Vec3::X,0.)}
	fn forward() -> Quat { Quat::from_axis_angle(Vec3::Z,0.)}
	fn back() -> Quat { Quat::from_axis_angle(-Vec3::Z,0.)}
}
