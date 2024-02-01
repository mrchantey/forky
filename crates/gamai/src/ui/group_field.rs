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

pub struct GroupField {
	pub display_name: String,
	pub children: Vec<FieldUi>,
}

impl GroupField {
	pub fn new(display_name: String, children: Vec<FieldUi>) -> Self {
		Self {
			display_name,
			children,
		}
	}
}
impl Display for GroupField {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("GroupField")
			.field("display_name", &self.display_name)
			.finish()
	}
}
