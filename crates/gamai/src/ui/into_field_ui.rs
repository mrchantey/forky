use super::*;
use crate::prelude::Score;
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
use strum::IntoEnumIterator;

/// Marker for fields

pub trait IntoFieldUi<ParentT: FieldParent, ValueT: FieldValue> {
	fn into_field_ui(
		reflect: FieldReflect<ParentT, ValueT>,
	) -> FieldUi<ParentT>;
}


impl<ParentT: FieldParent> IntoFieldUi<ParentT, bool> for bool {
	fn into_field_ui(reflect: FieldReflect<ParentT, bool>) -> FieldUi<ParentT> {
		CheckboxField { reflect }.into()
	}
}

impl<ParentT: FieldParent> IntoFieldUi<ParentT, String> for String {
	fn into_field_ui(
		reflect: FieldReflect<ParentT, String>,
	) -> FieldUi<ParentT> {
		TextField { reflect }.into()
	}
}




pub trait ItGoesIntoFieldUi<ParentT: FieldParent> {
	fn it_goes_into_field_ui(self) -> FieldUi<ParentT>;
}

impl<
		ParentT: FieldParent,
		T: Clone + Display + Reflect,
	> ItGoesIntoFieldUi<ParentT> for T
{
	fn it_goes_into_field_ui(self) -> FieldUi<ParentT> {
		{
			todo!()
			// T::into_field_ui(self.clone())
		}
	}
}
