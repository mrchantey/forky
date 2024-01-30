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

pub trait FieldParent: Reflect + Struct {}
impl<T: Reflect + Struct> FieldParent for T {}
pub trait FieldValue: Reflect + Clone + Display {}
impl<T: Reflect + Clone + Display> FieldValue for T {}


pub struct FieldReflect<ParentT: FieldParent, ValueT: FieldValue> {
	pub parent: Rc<RefCell<ParentT>>,
	pub name: String,
	pub display_name: String,
	pub phantom: std::marker::PhantomData<ValueT>,
	pub on_change: Option<Box<dyn Fn(&ParentT)>>,
}

const REFLECT_ERROR: &str =
	"FieldReflect: Field of this type not found, not good";

impl<ParentT: FieldParent, ValueT: FieldValue> FieldReflect<ParentT, ValueT> {
	pub fn new(
		parent: Rc<RefCell<ParentT>>,
		name: String,
		on_change: Option<Box<dyn Fn(&ParentT)>>,
	) -> Self {
		Self {
			display_name: heck::AsTitleCase(&name).to_string(),
			name,
			parent,
			on_change,
			phantom: std::marker::PhantomData,
		}
	}

	pub fn get(&self) -> ValueT {
		let parent = self.parent.borrow();
		parent
			.get_field::<ValueT>(&self.name)
			.expect(REFLECT_ERROR)
			.clone()
	}
	pub fn set(&self, value: ValueT) {
		let mut parent = self.parent.borrow_mut();
		*parent.get_field_mut(&self.name).expect(REFLECT_ERROR) = value;
		if let Some(on_change) = &self.on_change {
			on_change(&parent);
		}
	}
}

impl<ParentT: FieldParent, ValueT: FieldValue> Display
	for FieldReflect<ParentT, ValueT>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.display_name, self.get())
	}
}
