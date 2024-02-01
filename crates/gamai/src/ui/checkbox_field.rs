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

pub struct CheckboxField {
	pub reflect: FieldReflect<bool>,
}

impl Display for CheckboxField {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CheckboxField")
			.field("name", &self.reflect.field_name)
			.field("value", &self.reflect.get())
			.finish()
	}
}


impl Deref for CheckboxField {
	type Target = FieldReflect<bool>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl DerefMut for CheckboxField {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}
