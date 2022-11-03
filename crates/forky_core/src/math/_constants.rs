// use std::f32;
// pub use num_traits::*;
use num_traits::{Num, AsPrimitive};

pub const TAU: f32 = std::f32::consts::TAU;
pub const QUARTER_TAU: f32 = TAU * 0.25;
pub const HALF_TAU: f32 = TAU * 0.5;
pub const THREE_QUARTER_TAU: f32 = TAU * 0.25;

pub const PI: f32 = HALF_TAU;

// pub type F = f32;
pub fn f<T>(val:T)->f32 where T:Num + AsPrimitive<f32>{
	val.as_()
}