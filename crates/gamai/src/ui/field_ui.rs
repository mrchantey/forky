use crate::prelude::Tree;
use super::*;
use anyhow::Result;
use bevy_reflect::GetField;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectRef;
use bevy_reflect::Struct;
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

// #[derive(Display)]
pub enum FieldUi<T: FieldParent> {
	Group(GroupField<T>),
	Text(TextField<T>),
	Checkbox(CheckboxField<T>),
	Select(SelectField<T>),
	// number
	NumberF32(NumberField<T, f32>),
	NumberF64(NumberField<T, f64>),
	NumberI8(NumberField<T, i8>),
	NumberI16(NumberField<T, i16>),
	NumberI32(NumberField<T, i32>),
	NumberI64(NumberField<T, i64>),
	NumberU8(NumberField<T, u8>),
	NumberU16(NumberField<T, u16>),
	NumberU32(NumberField<T, u32>),
	NumberU64(NumberField<T, u64>),
	// slider
	SliderF32(SliderField<T, f32>),
	SliderF64(SliderField<T, f64>),
	SliderI8(SliderField<T, i8>),
	SliderI16(SliderField<T, i16>),
	SliderI32(SliderField<T, i32>),
	SliderI64(SliderField<T, i64>),
	SliderU8(SliderField<T, u8>),
	SliderU16(SliderField<T, u16>),
	SliderU32(SliderField<T, u32>),
	SliderU64(SliderField<T, u64>),
}
impl<T: FieldParent> Into<FieldUi<T>> for CheckboxField<T> {
	fn into(self) -> FieldUi<T> { FieldUi::Checkbox(self) }
}
impl<T: FieldParent> Into<FieldUi<T>> for TextField<T> {
	fn into(self) -> FieldUi<T> { FieldUi::Text(self) }
}
impl<T: FieldParent> Into<FieldUi<T>> for GroupField<T> {
	fn into(self) -> FieldUi<T> { FieldUi::Group(self) }
}

impl<T: FieldParent> FieldUi<T> {
	pub fn into_string_tree(&self)-> Tree<String>{
					match self{
			FieldUi::Group(val) => Tree{
				value: val.name.clone(),
				children: val.children.iter().map(|child| child.into_string_tree()).collect(),
			},
			FieldUi::Text(val) => Tree::new(val.reflect.to_string()),
			FieldUi::Checkbox(val) => Tree::new(val.reflect.to_string()),
			FieldUi::Select(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberF32(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberF64(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberI8(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberI16(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberI32(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberI64(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberU8(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberU16(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberU32(val) => Tree::new(val.reflect.to_string()),
			FieldUi::NumberU64(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderF32(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderF64(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderI8(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderI16(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderI32(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderI64(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderU8(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderU16(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderU32(val) => Tree::new(val.reflect.to_string()),
			FieldUi::SliderU64(val) => Tree::new(val.reflect.to_string()),
		}
	}

	pub fn parse(parent: &T) -> FieldUi<T> {
		let name = parent
			.get_represented_type_info()
			.unwrap()
			.type_path_table()
			.short_path()
			.to_string();

		Self::parse_inner(parent, name)
	}
	fn parse_inner(val: &dyn Reflect, name: String) -> FieldUi<T> {
		match val.reflect_ref() {
			ReflectRef::Struct(val) => FieldUi::Group(GroupField {
				name,
				children: val
					.iter_fields()
					.enumerate()
					.map(|(index, field)| {
						Self::parse_inner(
							field,
							val.name_at(index).unwrap().to_string(),
						)
					})
					.collect(),
			}),
			ReflectRef::TupleStruct(val) => FieldUi::Group(GroupField {
				name,
				children: val
					.iter_fields()
					.enumerate()
					.map(|(index, field)| {
						Self::parse_inner(field, index.to_string())
					})
					.collect(),
			}),
			ReflectRef::Tuple(val) => FieldUi::Group(GroupField {
				name,
				children: val
					.iter_fields()
					.enumerate()
					.map(|(index, field)| {
						Self::parse_inner(field, index.to_string())
					})
					.collect(),
			}),
			ReflectRef::List(val) => FieldUi::Group(GroupField {
				name,
				children: val
					.iter()
					.enumerate()
					.map(|(index, field)| {
						Self::parse_inner(field, index.to_string())
					})
					.collect(),
			}),
			ReflectRef::Array(val) => FieldUi::Group(GroupField {
				name,
				children: val
					.iter()
					.enumerate()
					.map(|(index, field)| {
						Self::parse_inner(field, index.to_string())
					})
					.collect(),
			}),
			ReflectRef::Map(val) => FieldUi::Group(GroupField {
				name,
				children: val
					.iter()
					.map(|(key, field)| {
						Self::parse_inner(
							field,
							key.downcast_ref::<String>().expect("Currently only maps with String keys are supported").clone(),
						)
					})
					.collect(),
			}),
			// // ReflectRef::Value(val)=> 
			// // ReflectRef::Enum(_) => todo!(),
			// ReflectRef::Value(val) => {
			// 	if let Some(val) = val.downcast_ref::<bool>(){


			// 	}

				
				
			// },
			_ => todo!(),
		}
	}
}


pub struct GroupField<T: FieldParent> {
	pub name: String,
	pub children: Vec<FieldUi<T>>,
}

// impl<T:FieldParent> ToIndentString for GroupField<T>{
// 		fn to_indent_string_recursive(&self,val: &mut String,indent:usize) {
// 			let indent_str = " ".repeat(indent);
// 			val.push_str(&format!("{indent_str}{}",self.name));
// 			for child in self.children.iter(){
// 				child.to_string_pretty_inner(val,indent+1);
// 			}
// 		}
// }

pub struct CheckboxField<T: FieldParent> {
	pub reflect: FieldReflect<T, bool>,
}

pub struct TextField<T: FieldParent> {
	pub reflect: FieldReflect<T, String>,
}


pub struct SliderField<T: FieldParent, ValueT: NumberFieldValue> {
	pub reflect: FieldReflect<T, ValueT>,
	pub min: ValueT,
	pub max: ValueT,
	pub step: ValueT,
}


impl<T: FieldParent> Deref for TextField<T> {
	type Target = FieldReflect<T, String>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl<T: FieldParent> DerefMut for TextField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent> Deref for CheckboxField<T> {
	type Target = FieldReflect<T, bool>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}
impl<T: FieldParent> DerefMut for CheckboxField<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}

impl<T: FieldParent, ValueT: NumberFieldValue> Deref
	for SliderField<T, ValueT>
{
	type Target = FieldReflect<T, ValueT>;
	fn deref(&self) -> &Self::Target { &self.reflect }
}

impl<T: FieldParent, ValueT: NumberFieldValue> DerefMut
	for SliderField<T, ValueT>
{
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.reflect }
}
