use std::{f32, i32, ops};
// use std::f32;
// use std::;
use extend::ext;

#[ext(name = OptI32X)]
pub impl Option<&i32> {
	fn or_default(&self) -> &i32 {
		match &self {
			Some(c) => c,
			None => &0,
		}
	}
}
// trait Bar {
// 	fn get(&self) -> usize;
// }


// impl ops::Add<i32> for f32 {
// 	type Output = f32;

// 	fn add(self, _rhs: i32) -> f32 { 0. }
// }
