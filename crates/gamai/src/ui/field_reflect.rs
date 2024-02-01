use anyhow::Result;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;


pub trait FieldValue: Clone {}
impl<T: Clone> FieldValue for T {}
pub type GetFunc<T> = Rc<Box<dyn Fn() -> T>>;
pub type SetFunc<T> = Rc<Box<dyn Fn(T)>>;


#[derive(Clone)]
pub struct FieldReflect<T: FieldValue> {
	pub field_name: String,
	pub display_name: String,
	get_cb: GetFunc<T>,
	set_cb: SetFunc<T>,
}

impl<T: FieldValue> FieldReflect<T> {
	pub fn new(
		field_name: String,
		get_cb: impl 'static + Fn() -> T,
		set_cb: impl 'static + Fn(T),
	) -> Self {
		Self {
			display_name: heck::AsTitleCase(&field_name).to_string(),
			field_name,
			get_cb: Rc::new(Box::new(get_cb)),
			set_cb: Rc::new(Box::new(set_cb)),
		}
	}

	pub fn clone_get_cb(&self) -> GetFunc<T> { self.get_cb.clone() }
	pub fn clone_set_cb(&self) -> SetFunc<T> { self.set_cb.clone() }

	pub fn get(&self) -> T { (self.get_cb)() }
	pub fn set(&self, value: T) { (self.set_cb)(value) }
}

impl<T: FieldValue + Display> Display for FieldReflect<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.display_name, self.get())
	}
}
