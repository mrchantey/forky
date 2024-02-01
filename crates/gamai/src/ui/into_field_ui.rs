use super::*;
use crate::prelude::Score;
use anyhow::Result;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use strum::IntoEnumIterator;

// Marker for fields

pub trait IntoFieldUi: 'static + Clone + Sized {
	fn into_field_ui_root(self) -> FieldUi {
		let this = Rc::new(RefCell::new(self));

		let reflect = FieldReflect::new(
			"root".to_string(),
			{
				let this = this.clone();
				move || this.borrow().clone()
			},
			{
				let this = this.clone();
				move |val| *this.borrow_mut() = val
			},
		);
		Self::into_field_ui(reflect)
	}
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi;
}
// pub trait IntoFieldUi<T: FieldValue> {
// 	fn into_field_ui(reflect: FieldReflect<T>) -> FieldUi;
// }


// impl IntoFieldUi<bool> for bool {
// 	fn into_field_ui(reflect: FieldReflect<bool>) -> FieldUi {
// 		CheckboxField { reflect }.into()
// 	}
// }

// impl IntoFieldUi<String> for String {
// 	fn into_field_ui(reflect: FieldReflect<String>) -> FieldUi {
// 		TextField { reflect }.into()
// 	}
// }
