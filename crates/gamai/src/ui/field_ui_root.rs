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
	pub on_change: Option<Rc<Box<dyn Fn(&T)>>>,
	pub on_ui_change: Option<Rc<Box<dyn Fn(FieldUi)>>>,
}


impl<T: IntoFieldUi> FieldUiRoot<T> {
	pub fn new(value: T) -> Self {
		let value = Rc::new(RefCell::new(value));
		Self {
			value,
			on_change: None,
			on_ui_change: None,
		}
	}

	pub fn with_on_change(mut self, on_change: impl Fn(&T) + 'static) -> Self {
		self.on_change = Some(Rc::new(Box::new(on_change)));
		self
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
			std::any::type_name::<T>().to_string(),
			// "root".to_string(),
			{
				let this = self.value.clone();
				move || this.borrow().clone()
			},
			{
				let this = self.clone();
				move |val| {
					let current_ui = this.get_ui();
					// set val
					*this.borrow_mut() = val;
					this.on_change.as_ref().map(|cb| cb(&this.borrow()));

					// handle ui
					let new_ui = this.get_ui();
					if false == current_ui.is_equal_graph(&new_ui) {
						this.on_ui_change.as_ref().map(|cb| cb(new_ui));
					}
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
