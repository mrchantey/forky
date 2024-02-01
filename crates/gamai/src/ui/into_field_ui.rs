use super::*;
use crate::prelude::Score;
use anyhow::Result;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::cell::Ref;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use strum::IntoEnumIterator;

// Marker for fields

#[derive(Clone)]
pub struct FieldUiRoot<T: IntoFieldUi> {
	pub value: Rc<RefCell<T>>,
	pub on_ui_change: Option<Rc<Box<dyn Fn(FieldUi)>>>,
}


impl<T: IntoFieldUi> FieldUiRoot<T> {
	pub fn new(value: T) -> Self {
		let value = Rc::new(RefCell::new(value));
		Self {
			value,
			on_ui_change: None,
		}
	}

	pub fn with_on_ui_change(
		mut self,
		on_ui_change: impl Fn(FieldUi) + 'static,
	) -> Self {
		self.on_ui_change = Some(Rc::new(Box::new(on_ui_change)));
		self
	}

	pub fn get_ui(&self) -> FieldUi {
		let reflect = FieldReflect::new(
			"root".to_string(),
			{
				let this = self.value.clone();
				move || this.borrow().clone()
			},
			{
				let this = self.clone();
				move |val| {
					*this.borrow_mut() = val;
					let new_ui = this.get_ui();
					this.on_ui_change.as_ref().map(|cb| cb(new_ui));
					//TODO check if ui needs refresh
				}
			},
		);
		T::into_field_ui(reflect)
	}
}
impl<T: IntoFieldUi> Deref for FieldUiRoot<T> {
	type Target = Rc<RefCell<T>>;
	fn deref(&self) -> &Self::Target { &self.value }
}
impl<T: IntoFieldUi> DerefMut for FieldUiRoot<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}


pub trait IntoFieldUi: 'static + Clone + Sized {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi;
}
// pub trait IntoFieldUi<T: FieldValue> {
// 	fn into_field_ui(reflect: FieldReflect<T>) -> FieldUi;
// }

impl IntoFieldUi for bool {
	fn into_field_ui(reflect: FieldReflect<bool>) -> FieldUi {
		CheckboxField { reflect }.into()
	}
}

impl IntoFieldUi for String {
	fn into_field_ui(reflect: FieldReflect<String>) -> FieldUi {
		TextField { reflect }.into()
	}
}
