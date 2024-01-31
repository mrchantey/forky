use super::*;
use crate::prelude::Score;
use bevy_reflect::Reflect;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use strum::IntoEnumIterator;

pub trait SelectFieldValue:
	Clone + Display + Reflect + IntoEnumIterator
{
}

pub trait SelectFieldReflect<T: FieldParent> {
	// fn option(&self) -> String;
	fn display_name(&self) -> &str;
	fn options(&self) -> Vec<String>;
	fn get_index(&self) -> usize;
	fn set_index(&self, index: usize);
}

impl<T: FieldParent, V: SelectFieldValue> SelectFieldReflect<T>
	for FieldReflect<T, V>
{
	fn display_name(&self) -> &str { &self.display_name }

	fn options(&self) -> Vec<String> {
		V::iter().map(|s| s.to_string()).collect()
	}
	fn get_index(&self) -> usize {
		let a = self.get().to_string();
		V::iter().position(|s| s.to_string() == a).unwrap()
	}
	fn set_index(&self, index: usize) {
		let options = V::iter().collect::<Vec<_>>();
		self.set(options[index].clone());
	}
}

pub struct SelectField<T: FieldParent> {
	pub reflect: Box<dyn SelectFieldReflect<T>>,
}

impl<T: FieldParent> Deref for SelectField<T> {
	type Target = Box<dyn SelectFieldReflect<T>>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: FieldParent> DerefMut for SelectField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent> Into<FieldUi<T>> for SelectField<T> {
	fn into(self) -> FieldUi<T> { FieldUi::Select(self) }
}

impl<ParentT: FieldParent, ValueT: SelectFieldValue>
	IntoFieldUi<ParentT, ValueT> for ValueT
{
	fn into_field_ui(
		reflect: FieldReflect<ParentT, ValueT>,
	) -> FieldUi<ParentT> {
		SelectField {
			reflect: Box::new(reflect),
		}
		.into()
	}
}

impl<ParentT: FieldParent> Display for Box<dyn SelectFieldReflect<ParentT>> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {}",
			self.display_name(),
			self.options()[self.get_index()]
		)
	}
}
