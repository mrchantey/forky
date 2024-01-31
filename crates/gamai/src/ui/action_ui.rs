use super::*;
use anyhow::Result;
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

#[derive(Reflect)]
pub struct ActionUi<T: Struct> {
	pub label: String,
	pub value: Rc<RefCell<T>>,
	pub fields: Vec<FieldUi<T>>,
}

// impl<T: Struct> Display for ActionUi<T> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{}", self.label)?;
// 		for field in &self.fields {
// 			write!(f, "\n  {}", field)?;
// 		}
// 		Ok(())
// 	}
// }

pub trait IntoActionUi<T: Struct> {
	fn into_action_ui(
		self,
		on_change: impl 'static + Clone + Fn(&Self),
	) -> ActionUi<T>;
}
