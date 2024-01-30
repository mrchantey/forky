use super::*;
use anyhow::Result;
use bevy_reflect::GetField;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;


pub trait NumberValueT: PartialOrd + Display {}
impl<T: PartialOrd + Display> NumberValueT for T {}



// #[derive(Display)]
pub enum FieldUi<T: FieldParent> {
	// Group(GroupProp),
	Text(TextField<T>),
	Checkbox(CheckboxField<T>),
	Select(SelectField<T>),
	// number
	NumberF32(NumberField<T, f32>),
	NumberF64(NumberField<T, f64>),
	NumberI8(NumberField<T, i8>),
	NumberI16(NumberField<T, i16>),
	NumberI32(NumberField<T, i32>),
	NumberI64(NumberField<T, i64>),
	NumberU8(NumberField<T, u8>),
	NumberU16(NumberField<T, u16>),
	NumberU32(NumberField<T, u32>),
	NumberU64(NumberField<T, u64>),
	// slider
	SliderF32(SliderField<T, f32>),
	SliderF64(SliderField<T, f64>),
	SliderI8(SliderField<T, i8>),
	SliderI16(SliderField<T, i16>),
	SliderI32(SliderField<T, i32>),
	SliderI64(SliderField<T, i64>),
	SliderU8(SliderField<T, u8>),
	SliderU16(SliderField<T, u16>),
	SliderU32(SliderField<T, u32>),
	SliderU64(SliderField<T, u64>),
}
impl<T: FieldParent> Into<FieldUi<T>> for CheckboxField<T> {
	fn into(self) -> FieldUi<T> { FieldUi::Checkbox(self) }
}
impl<T: FieldParent> Into<FieldUi<T>> for TextField<T> {
	fn into(self) -> FieldUi<T> { FieldUi::Text(self) }
}

// impl Into<PropUi> for GroupProp {
// 	fn into(self) -> PropUi { PropUi::Group(self) }
// }

impl<T: FieldParent> Display for FieldUi<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FieldUi::Text(val) => val.fmt(f),
			FieldUi::Checkbox(val) => val.fmt(f),
			FieldUi::Select(val) => val.fmt(f),
			// number
			FieldUi::NumberF32(val) => val.fmt(f),
			FieldUi::NumberF64(val) => val.fmt(f),
			FieldUi::NumberI8(val) => val.fmt(f),
			FieldUi::NumberI16(val) => val.fmt(f),
			FieldUi::NumberI32(val) => val.fmt(f),
			FieldUi::NumberI64(val) => val.fmt(f),
			FieldUi::NumberU8(val) => val.fmt(f),
			FieldUi::NumberU16(val) => val.fmt(f),
			FieldUi::NumberU32(val) => val.fmt(f),
			FieldUi::NumberU64(val) => val.fmt(f),
			// slider
			FieldUi::SliderF32(val) => val.fmt(f),
			FieldUi::SliderF64(val) => val.fmt(f),
			FieldUi::SliderI8(val) => val.fmt(f),
			FieldUi::SliderI16(val) => val.fmt(f),
			FieldUi::SliderI32(val) => val.fmt(f),
			FieldUi::SliderI64(val) => val.fmt(f),
			FieldUi::SliderU8(val) => val.fmt(f),
			FieldUi::SliderU16(val) => val.fmt(f),
			FieldUi::SliderU32(val) => val.fmt(f),
			FieldUi::SliderU64(val) => val.fmt(f),
		}
	}
}

pub struct CheckboxField<T: FieldParent> {
	pub reflect: FieldReflect<T, bool>,
}

pub struct TextField<T: FieldParent> {
	pub reflect: FieldReflect<T, String>,
}

pub struct SelectField<T: FieldParent> {
	pub reflect: FieldReflect<T, usize>,
	pub options: Vec<String>,
}

pub struct NumberField<T: FieldParent, ValueT: PartialOrd> {
	pub reflect: FieldReflect<T, String>,
	pub step: ValueT,
}
pub struct SliderField<T: FieldParent, ValueT: NumberValueT> {
	pub reflect: FieldReflect<T, String>,
	pub min: ValueT,
	pub max: ValueT,
	pub step: ValueT,
}


impl<T: FieldParent> Deref for TextField<T> {
	type Target = FieldReflect<T, String>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl<T: FieldParent> DerefMut for TextField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent> Deref for CheckboxField<T> {
	type Target = FieldReflect<T, bool>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl<T: FieldParent> DerefMut for CheckboxField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent> Deref for SelectField<T> {
	type Target = FieldReflect<T, usize>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: FieldParent> DerefMut for SelectField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent, ValueT: NumberValueT> Deref for NumberField<T, ValueT> {
	type Target = FieldReflect<T, String>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl<T: FieldParent, ValueT: NumberValueT> DerefMut for NumberField<T, ValueT> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent, ValueT: NumberValueT> Deref for SliderField<T, ValueT> {
	type Target = FieldReflect<T, String>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: FieldParent, ValueT: NumberValueT> DerefMut for SliderField<T, ValueT> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}
