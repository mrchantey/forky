use super::*;
use anyhow::Result;
use bevy_reflect::GetField;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
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

pub trait NumberFieldValue: Clone + PartialOrd + Display + Reflect {}
impl<T: Clone + PartialOrd + Display + Reflect> NumberFieldValue for T {}


pub struct NumberField<T: FieldParent, ValueT: NumberFieldValue> {
	pub reflect: FieldReflect<T, ValueT>,
	pub step: ValueT,
}
impl<T: FieldParent, ValueT: NumberFieldValue> Deref
	for NumberField<T, ValueT>
{
	type Target = FieldReflect<T, ValueT>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: FieldParent, ValueT: NumberFieldValue> DerefMut
	for NumberField<T, ValueT>
{
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}





impl<ParentT: FieldParent> IntoFieldUi<ParentT, u8> for u8 {
	fn into_field_ui(reflect: FieldReflect<ParentT, u8>) -> FieldUi<ParentT> {
		FieldUi::NumberU8(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, u16> for u16 {
	fn into_field_ui(reflect: FieldReflect<ParentT, u16>) -> FieldUi<ParentT> {
		FieldUi::NumberU16(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, u32> for u32 {
	fn into_field_ui(reflect: FieldReflect<ParentT, u32>) -> FieldUi<ParentT> {
		FieldUi::NumberU32(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, u64> for u64 {
	fn into_field_ui(reflect: FieldReflect<ParentT, u64>) -> FieldUi<ParentT> {
		FieldUi::NumberU64(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, i8> for i8 {
	fn into_field_ui(reflect: FieldReflect<ParentT, i8>) -> FieldUi<ParentT> {
		FieldUi::NumberI8(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, i16> for i16 {
	fn into_field_ui(reflect: FieldReflect<ParentT, i16>) -> FieldUi<ParentT> {
		FieldUi::NumberI16(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, i32> for i32 {
	fn into_field_ui(reflect: FieldReflect<ParentT, i32>) -> FieldUi<ParentT> {
		FieldUi::NumberI32(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, i64> for i64 {
	fn into_field_ui(reflect: FieldReflect<ParentT, i64>) -> FieldUi<ParentT> {
		FieldUi::NumberI64(NumberField { reflect, step: 1 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, f32> for f32 {
	fn into_field_ui(reflect: FieldReflect<ParentT, f32>) -> FieldUi<ParentT> {
		FieldUi::NumberF32(NumberField { reflect, step: 1.0 })
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, f64> for f64 {
	fn into_field_ui(reflect: FieldReflect<ParentT, f64>) -> FieldUi<ParentT> {
		FieldUi::NumberF64(NumberField { reflect, step: 1.0 })
	}
}