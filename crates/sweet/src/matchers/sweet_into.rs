#![cfg_attr(rustfmt, rustfmt_skip)]


// TODO deprecate sweetinto and sweetborrow in favor of this
pub trait Sweet<T,M>{
	fn as_sweet(self) -> T;
}

pub struct SelfSweetMarker;
pub struct CloneSweetMarker;
pub struct IntoSweetMarker;

impl<I,O> Sweet<O,IntoSweetMarker> for I where I:Into<O> {
	fn as_sweet(self) -> O { self.into() }
}
impl<I,O> Sweet<O,CloneSweetMarker> for &I where I:Clone + Into<O> {
	fn as_sweet(self) -> O { self.clone().into() }
}


pub trait SweetInto<T> {
	fn sweet_into(self) -> T;
}

impl<T> SweetInto<T> for T {
	fn sweet_into(self) -> T { self }
}

pub trait SweetBorrow<T> {
	fn sweet_borrow(&self) -> T;
}

// all SweetInto + Clone are SweetBorrow
impl<T,U> SweetBorrow<U> for T where T : SweetInto<U> + Clone,
{ fn sweet_borrow(&self) -> U { self.clone().sweet_into() } }


impl<T> SweetInto<i8> for &T where T: std::ops::Deref<Target = i8>,
{ fn sweet_into(self) -> i8 { **self } }
impl<T> SweetInto<i16> for &T where T: std::ops::Deref<Target = i16>,
{ fn sweet_into(self) -> i16 { **self } }
impl<T> SweetInto<i32> for &T where T: std::ops::Deref<Target = i32>,
{ fn sweet_into(self) -> i32 { **self } }
impl<T> SweetInto<i64> for &T where T: std::ops::Deref<Target = i64>,
{ fn sweet_into(self) -> i64 { **self } }
impl<T> SweetInto<i128> for &T where T: std::ops::Deref<Target = i128>,
{ fn sweet_into(self) -> i128 { **self } }
impl<T> SweetInto<u8> for &T where T: std::ops::Deref<Target = u8>,
{ fn sweet_into(self) -> u8 { **self } }
impl<T> SweetInto<u16> for &T where T: std::ops::Deref<Target = u16>,
{ fn sweet_into(self) -> u16 { **self } }
impl<T> SweetInto<u32> for &T where T: std::ops::Deref<Target = u32>,
{ fn sweet_into(self) -> u32 { **self } }
impl<T> SweetInto<u64> for &T where T: std::ops::Deref<Target = u64>,
{ fn sweet_into(self) -> u64 { **self } }
impl<T> SweetInto<u128> for &T where T: std::ops::Deref<Target = u128>,
{ fn sweet_into(self) -> u128 { **self } }

impl<T> SweetInto<f32> for &T where T: std::ops::Deref<Target = f32>,
{ fn sweet_into(self) -> f32 { **self } }
impl<T> SweetInto<f64> for &T where T: std::ops::Deref<Target = f64>,
{ fn sweet_into(self) -> f64 { **self } }

impl<T> SweetInto<bool> for &T where T: std::ops::Deref<Target = bool>,
{ fn sweet_into(self) -> bool { **self } }
impl<'a,T> SweetInto<&'a str> for &'a T where T: std::ops::Deref<Target = &'a str>,
{ fn sweet_into(self) -> &'a str { **self } }
impl<T> SweetInto<String> for &T where T: std::ops::Deref<Target = String>,
{ fn sweet_into(self) -> String { (*self).clone() } }


// impl SweetInto<f64> for f32
// { fn sweet_into(self) -> f64 { self.into() } }
// impl<T> SweetInto<f64> for &T where T: std::ops::Deref<Target = f32>,
// { fn sweet_into(self) -> f64 { (**self).into() } }

// impl<T,U> SweetInto<U> for T
// where
// 	T: std::ops::Deref<Target = U>,
// 	U:Clone
// {
// 	fn sweet_into(&self) -> U { (**self).clone() }
// }
// impl<T, U> SweetInto<U> for T
// where
// 	T: Into<U> + Clone,
// {
// 	fn sweet_into(&self) -> U { (*self).clone().into() }
// }
