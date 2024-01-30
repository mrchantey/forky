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


pub trait IntoFieldUi<T: FieldParent>
where
	Self: FieldValue,
{
	fn into_field_ui(reflect: FieldReflect<T, Self>) -> FieldUi<T>;
}


impl<T: FieldParent> IntoFieldUi<T> for bool {
	fn into_field_ui(reflect: FieldReflect<T, Self>) -> FieldUi<T> {
		CheckboxField { reflect }.into()
	}
}

impl<T: FieldParent> IntoFieldUi<T> for String {
	fn into_field_ui(reflect: FieldReflect<T, Self>) -> FieldUi<T> {
		TextField { reflect }.into()
	}
}
