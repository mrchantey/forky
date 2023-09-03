#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::*;
use ::bevy::prelude::*;

impl<T> SweetInto<Vec2> for &T where T: std::ops::Deref<Target = Vec2>,
{ fn sweet_into(self) -> Vec2 { **self } }
impl<T> SweetInto<Vec3> for &T where T: std::ops::Deref<Target = Vec3>,
{ fn sweet_into(self) -> Vec3 { **self } }
impl<T> SweetInto<Quat> for &T where T: std::ops::Deref<Target = Quat>,
{ fn sweet_into(self) -> Quat { **self } }