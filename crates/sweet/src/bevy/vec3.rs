use extend::ext;
use crate::*;
use ::bevy::prelude::*;
use anyhow::Result;

#[ext]
pub impl Matcher<&Vec3> {
	fn to_be_close_to(&self, expected: Vec3) -> Result<()> {
		self.to_be_close_to_with_epsilon(expected, DEFAULT_EPSILON_F32)
	}
	fn to_be_close_to_with_epsilon(&self, expected: Vec3,epsilon:f32) -> Result<()> {
		let x = is_close_f32(self.value.x, expected.x, epsilon);
		let y = is_close_f32(self.value.y, expected.y, epsilon);
		let z = is_close_f32(self.value.z, expected.z, epsilon);
		let result = x && y && z;
		let expected = format!("close to {:?}", expected);
		self.assert_correct(result, &expected)
	}
}
