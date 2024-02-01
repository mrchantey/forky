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


pub struct SliderField<ValueT: NumberFieldValue> {
	pub reflect: FieldReflect<ValueT>,
	pub min: ValueT,
	pub max: ValueT,
	pub step: ValueT,
}

impl<ValueT: NumberFieldValue> SliderField<ValueT> {
	pub fn new(
		field_name: String,
		get_cb: impl 'static + Fn() -> ValueT,
		set_cb: impl 'static + Fn(ValueT),
		min: ValueT,
		max: ValueT,
		step: ValueT,
	) -> Self {
		Self {
			reflect: FieldReflect::new(field_name, get_cb, set_cb),
			min,
			max,
			step,
		}
	}
}

impl<T: NumberFieldValue> Deref for SliderField<T> {
	type Target = FieldReflect<T>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: NumberFieldValue> DerefMut for SliderField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: NumberFieldValue> Display for SliderField<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SliderField")
			.field("name", &self.reflect.field_name)
			.field("value", &self.reflect.get().to_string())
			.field("min", &self.min.to_string())
			.field("max", &self.max.to_string())
			.field("step", &self.step.to_string())
			.finish()
	}
}