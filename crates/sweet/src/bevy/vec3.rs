use crate::*;
use ::bevy::prelude::*;


impl CloseTo for Vec3 {
	fn default_epsilon() -> Self {
		Vec3::new(
			DEFAULT_EPSILON_F32,
			DEFAULT_EPSILON_F32,
			DEFAULT_EPSILON_F32,
		)
	}
	fn is_close(a: Self, b: Self, epsilon: Self) -> bool {
		is_close_f32(a.x, b.x, epsilon.x)
			&& is_close_f32(a.y, b.y, epsilon.y)
			&& is_close_f32(a.z, b.z, epsilon.z)
	}
}
