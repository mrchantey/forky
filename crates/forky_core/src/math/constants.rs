// use std::f32;
// pub use num_traits::*;
use num_traits::{AsPrimitive, Num};

pub const TAU: f32 = std::f32::consts::TAU;
pub const QUARTER_TAU: f32 = TAU * 0.25;
pub const HALF_TAU: f32 = TAU * 0.5;
pub const THREE_QUARTER_TAU: f32 = TAU * 0.75;

pub const PI: f32 = HALF_TAU;

pub const GRAVITY: f32 = -9.81;


pub const DEG2RAD: f32 = PI / 180.;
pub const RAD2DEG: f32 = 180. / PI;
pub const RAD2HOURS: f32 = 12. / PI;
pub const HOURS2RAD: f32 = PI / 12.;
pub const DEG2HOURS: f32 = 1. / 15.;
pub const HOURS2DEG: f32 = 15.;

// pub type F = f32;
pub fn f<T>(val: T) -> f32
where
	T: Num + AsPrimitive<f32>,
{
	val.as_()
}
// pub fn i<T>(val:T)->i32 where T:Num + AsPrimitive<i32>{
// 	val.as_()
// }
