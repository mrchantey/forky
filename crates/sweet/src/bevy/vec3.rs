use crate::*;
use ::bevy::prelude::*;
use anyhow::Result;
use extend::ext;

impl SweetInto<Vec3> for Vec3 {
	fn sweet_into(&self) -> Vec3 { (*self).clone() }
}

impl<T, U> SweetInto<Vec3> for &T
where
	T: std::ops::Deref<Target = U>,
	U: Into<Vec3> + Clone,
{
	fn sweet_into(&self) -> Vec3 { (**self).clone().into() }
}

#[ext]
pub impl<T> Matcher<T>
where
	T: SweetInto<Vec3>,
{
	fn to_be_close_to(&self, expected: Vec3) -> Result<()> {
		self.to_be_close_to_with_epsilon(expected, DEFAULT_EPSILON_F32)
	}
	fn to_be_close_to_with_epsilon(
		&self,
		expected: Vec3,
		epsilon: f32,
	) -> Result<()> {
		let value = self.value.sweet_into();
		let x = is_close_f32(value.x, expected.x, epsilon);
		let y = is_close_f32(value.y, expected.y, epsilon);
		let z = is_close_f32(value.z, expected.z, epsilon);
		let result = x && y && z;
		let expected = format!("within {epsilon} of {:?}", expected);
		self.assert_correct_with_received(result, &expected, &value)
	}
}
// impl SweetInto<Vec3> for &Vec3 {
// 	fn sweet_into(&self) -> Vec3 { (**self).clone() }
// }

// impl<T,U> SweetInto<Vec3> for T
// where
// 	T: std::ops::Deref<Target = U>,
// 	U: SweetInto<Vec3>
// {
// 	fn sweet_into(&self) -> Vec3 { (**self).sweet_into() }
// }



// impl<T> SweetInto<Vec3> for T
// where
// 	T: std::ops::Deref<Target = Vec3>,
// {
// 	fn sweet_into(&self) -> Vec3 { (**self).clone() }
// }
