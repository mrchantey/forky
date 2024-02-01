use super::*;
use crate::prelude::Tree;
use anyhow::Result;
use bevy_utils::tracing::Value;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub struct TextField {
	pub reflect: FieldReflect<String>,
}
impl Display for TextField {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TextField")
			.field("name", &self.reflect.field_name)
			.field("value", &self.reflect.get())
			.finish()
	}
}


impl Deref for TextField {
	type Target = FieldReflect<String>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl DerefMut for TextField {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}
