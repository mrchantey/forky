use anyhow::Result;
use bevy_reflect::GetField;
use bevy_reflect::ParsedPath;
use bevy_reflect::Reflect;
use bevy_reflect::ReflectPath;
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
pub trait FieldValue: Reflect + Clone {}
impl<T: Reflect + Clone> FieldValue for T {}

pub struct FieldReflect<ParentT: FieldParent, ValueT: FieldValue> {
	pub root: Rc<RefCell<ParentT>>,
	pub name: String,
	pub path: ParsedPath,
	pub display_name: String,
	pub phantom: std::marker::PhantomData<ValueT>,
	pub on_change: Option<Box<dyn Fn(&ParentT)>>,
}

const REFLECT_ERROR: &str =
	"FieldReflect: Field of this type not found, not good";

impl<ParentT: FieldParent, ValueT: FieldValue> FieldReflect<ParentT, ValueT> {
	pub fn new(
		root: Rc<RefCell<ParentT>>,
		name: String,
		path: ParsedPath,
		on_change: Option<Box<dyn Fn(&ParentT)>>,
	) -> Self {
		Self {
			display_name: heck::AsTitleCase(&name).to_string(),
			path,
			name,
			root,
			on_change,
			phantom: std::marker::PhantomData,
		}
	}

	pub fn get(&self) -> ValueT {
		let parent = self.root.borrow();
		self.path
			.element::<ValueT>(&*parent)
			.expect(REFLECT_ERROR)
			.clone()
		// parent
		// 	.get_field::<ValueT>(&self.name)
		// 	.expect(REFLECT_ERROR)
		// 	.clone()
	}
	pub fn set(&self, value: ValueT) {
		let mut parent = self.root.borrow_mut();
		// *parent
		*self.path.element_mut(&mut *parent).expect(REFLECT_ERROR) = value;
		if let Some(on_change) = &self.on_change {
			on_change(&parent);
		}
	}
}

impl<ParentT: FieldParent, ValueT: FieldValue + Display> Display
	for FieldReflect<ParentT, ValueT>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.display_name, self.get())
	}
}
