use super::*;
use crate::prelude::Score;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use strum::IntoEnumIterator;

pub trait SelectFieldValue: FieldValue + Display + IntoEnumIterator {}
impl<T: FieldValue + Display + IntoEnumIterator> SelectFieldValue for T {}

pub struct SelectField {
	pub options: Vec<String>,
	pub reflect: FieldReflect<usize>,
}

impl SelectField {
	pub fn new<T: 'static + SelectFieldValue>(
		field_name: String,
		get_cb: GetFunc<T>,
		set_cb: SetFunc<T>,
	) -> Self {
		Self {
			options: T::iter().map(|s| s.to_string()).collect(),
			reflect: FieldReflect::new(
				field_name,
				move || {
					let a = get_cb().to_string();
					T::iter().position(|s| s.to_string() == a).unwrap()
				},
				move |index| {
					let options = T::iter().collect::<Vec<_>>();
					set_cb(options[index].clone());
				},
			),
		}
	}
	pub fn selected_option(&self) -> String {
		self.options[self.reflect.get()].clone()
	}
}


impl Into<FieldUi> for SelectField {
	fn into(self) -> FieldUi { FieldUi::Select(self) }
}


impl Display for SelectField {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SelectField")
			.field("name", &self.reflect.field_name)
			.field("value", &self.selected_option())
			.field("index", &self.reflect.get())
			.field("options", &self.options)
			.finish()
	}
}

// impl IntoFieldUi<ValueT> for ValueT {
// 	fn into_field_ui(reflect: FieldReflect<ValueT>) -> FieldUi {
// 		SelectField {
// 			reflect: Box::new(reflect),
// 		}
// 		.into()
// 	}
// }
