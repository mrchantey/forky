use crate::*;
use extend::ext;
use smart_leds::{
	brightness, gamma,
	hsv::{hsv2rgb, Hsv},
	SmartLedsWrite, RGB, RGB8,
};
use std::{fmt::Display, slice::SliceIndex};

// #[ext]
// pub impl RGBW8 {
// 	fn to_rgb(&self) -> RGB8 {
// 		RGB8{
// 			self.r,
// 			self.g,
// 			self.b
// 		}
// 	}
// }
