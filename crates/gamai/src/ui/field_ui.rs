use super::*;
use crate::prelude::Tree;
use anyhow::Result;
use bevy_utils::tracing::Value;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use strum::IntoEnumIterator;

// #[derive(Display)]
pub enum FieldUi {
	Group(GroupField),
	Text(TextField),
	Checkbox(CheckboxField),
	Select(SelectField),
	// number
	NumberF32(NumberField<f32>),
	NumberF64(NumberField<f64>),
	NumberI8(NumberField<i8>),
	NumberI16(NumberField<i16>),
	NumberI32(NumberField<i32>),
	NumberI64(NumberField<i64>),
	NumberU8(NumberField<u8>),
	NumberU16(NumberField<u16>),
	NumberU32(NumberField<u32>),
	NumberU64(NumberField<u64>),
	// slider
	SliderF32(SliderField<f32>),
	SliderF64(SliderField<f64>),
	SliderI8(SliderField<i8>),
	SliderI16(SliderField<i16>),
	SliderI32(SliderField<i32>),
	SliderI64(SliderField<i64>),
	SliderU8(SliderField<u8>),
	SliderU16(SliderField<u16>),
	SliderU32(SliderField<u32>),
	SliderU64(SliderField<u64>),
}

impl FieldUi {
	pub fn into_string_tree(&self) -> Tree<String> {
		match self {
			FieldUi::Group(val) => Tree {
				value: val.to_string(),
				children: val
					.children
					.iter()
					.map(|child| child.into_string_tree())
					.collect(),
			},
			FieldUi::Text(val) => Tree::new(val.to_string()),
			FieldUi::Checkbox(val) => Tree::new(val.to_string()),
			FieldUi::Select(val) => Tree::new(val.to_string()),
			FieldUi::NumberF32(val) => Tree::new(val.to_string()),
			FieldUi::NumberF64(val) => Tree::new(val.to_string()),
			FieldUi::NumberI8(val) => Tree::new(val.to_string()),
			FieldUi::NumberI16(val) => Tree::new(val.to_string()),
			FieldUi::NumberI32(val) => Tree::new(val.to_string()),
			FieldUi::NumberI64(val) => Tree::new(val.to_string()),
			FieldUi::NumberU8(val) => Tree::new(val.to_string()),
			FieldUi::NumberU16(val) => Tree::new(val.to_string()),
			FieldUi::NumberU32(val) => Tree::new(val.to_string()),
			FieldUi::NumberU64(val) => Tree::new(val.to_string()),
			FieldUi::SliderF32(val) => Tree::new(val.to_string()),
			FieldUi::SliderF64(val) => Tree::new(val.to_string()),
			FieldUi::SliderI8(val) => Tree::new(val.to_string()),
			FieldUi::SliderI16(val) => Tree::new(val.to_string()),
			FieldUi::SliderI32(val) => Tree::new(val.to_string()),
			FieldUi::SliderI64(val) => Tree::new(val.to_string()),
			FieldUi::SliderU8(val) => Tree::new(val.to_string()),
			FieldUi::SliderU16(val) => Tree::new(val.to_string()),
			FieldUi::SliderU32(val) => Tree::new(val.to_string()),
			FieldUi::SliderU64(val) => Tree::new(val.to_string()),
		}
	}

	pub fn is_equal_graph(&self, other: &FieldUi) -> bool {
		match (self, other) {
			(FieldUi::Group(val), FieldUi::Group(other)) => {
				val.display_name == other.display_name
					&& val.children.len() == other.children.len()
					&& val
						.children
						.iter()
						.zip(other.children.iter())
						.all(|(a, b)| a.is_equal_graph(b))
			}
			(FieldUi::Text(val), FieldUi::Text(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::Checkbox(val), FieldUi::Checkbox(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::Select(val), FieldUi::Select(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberF32(val), FieldUi::NumberF32(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberF64(val), FieldUi::NumberF64(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberI8(val), FieldUi::NumberI8(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberI16(val), FieldUi::NumberI16(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberI32(val), FieldUi::NumberI32(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberI64(val), FieldUi::NumberI64(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberU8(val), FieldUi::NumberU8(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberU16(val), FieldUi::NumberU16(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberU32(val), FieldUi::NumberU32(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::NumberU64(val), FieldUi::NumberU64(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderF32(val), FieldUi::SliderF32(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderF64(val), FieldUi::SliderF64(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderI8(val), FieldUi::SliderI8(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderI16(val), FieldUi::SliderI16(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderI32(val), FieldUi::SliderI32(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderI64(val), FieldUi::SliderI64(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderU8(val), FieldUi::SliderU8(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderU16(val), FieldUi::SliderU16(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderU32(val), FieldUi::SliderU32(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			(FieldUi::SliderU64(val), FieldUi::SliderU64(other)) => {
				val.reflect.field_name == other.reflect.field_name
					&& val.reflect.get() == other.reflect.get()
			}
			_ => false,
		}
	}
}


impl Into<FieldUi> for CheckboxField {
	fn into(self) -> FieldUi { FieldUi::Checkbox(self) }
}
impl Into<FieldUi> for TextField {
	fn into(self) -> FieldUi { FieldUi::Text(self) }
}
impl Into<FieldUi> for GroupField {
	fn into(self) -> FieldUi { FieldUi::Group(self) }
}
impl Into<FieldUi> for NumberField<u8> {
	fn into(self) -> FieldUi { FieldUi::NumberU8(self) }
}
impl Into<FieldUi> for NumberField<u16> {
	fn into(self) -> FieldUi { FieldUi::NumberU16(self) }
}
impl Into<FieldUi> for NumberField<u32> {
	fn into(self) -> FieldUi { FieldUi::NumberU32(self) }
}
impl Into<FieldUi> for NumberField<u64> {
	fn into(self) -> FieldUi { FieldUi::NumberU64(self) }
}
impl Into<FieldUi> for NumberField<i8> {
	fn into(self) -> FieldUi { FieldUi::NumberI8(self) }
}
impl Into<FieldUi> for NumberField<i16> {
	fn into(self) -> FieldUi { FieldUi::NumberI16(self) }
}
impl Into<FieldUi> for NumberField<i32> {
	fn into(self) -> FieldUi { FieldUi::NumberI32(self) }
}
impl Into<FieldUi> for NumberField<i64> {
	fn into(self) -> FieldUi { FieldUi::NumberI64(self) }
}
impl Into<FieldUi> for NumberField<f32> {
	fn into(self) -> FieldUi { FieldUi::NumberF32(self) }
}
impl Into<FieldUi> for NumberField<f64> {
	fn into(self) -> FieldUi { FieldUi::NumberF64(self) }
}
impl Into<FieldUi> for SliderField<u8> {
	fn into(self) -> FieldUi { FieldUi::SliderU8(self) }
}
impl Into<FieldUi> for SliderField<u16> {
	fn into(self) -> FieldUi { FieldUi::SliderU16(self) }
}
impl Into<FieldUi> for SliderField<u32> {
	fn into(self) -> FieldUi { FieldUi::SliderU32(self) }
}
impl Into<FieldUi> for SliderField<u64> {
	fn into(self) -> FieldUi { FieldUi::SliderU64(self) }
}
impl Into<FieldUi> for SliderField<i8> {
	fn into(self) -> FieldUi { FieldUi::SliderI8(self) }
}
impl Into<FieldUi> for SliderField<i16> {
	fn into(self) -> FieldUi { FieldUi::SliderI16(self) }
}
impl Into<FieldUi> for SliderField<i32> {
	fn into(self) -> FieldUi { FieldUi::SliderI32(self) }
}
impl Into<FieldUi> for SliderField<i64> {
	fn into(self) -> FieldUi { FieldUi::SliderI64(self) }
}
impl Into<FieldUi> for SliderField<f32> {
	fn into(self) -> FieldUi { FieldUi::SliderF32(self) }
}
impl Into<FieldUi> for SliderField<f64> {
	fn into(self) -> FieldUi { FieldUi::SliderF64(self) }
}
