use anyhow::Result;
use gamai::action::IntoAction;
use petgraph::graph::DiGraph;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

pub trait Prop<T>: Deref<Target = T> + DerefMut<Target = T> {}
impl<T: Deref<Target = T> + DerefMut<Target = T>> Prop<T> for T {}


// pub struct GraphFields<T: IntoAction> {
// 	pub graph: DiGraph<Vec<T>, ()>,

// }


pub struct ActionUi<T> {
	pub label: String,
	pub value: Rc<RefCell<T>>,
	pub fields: Vec<PropUi>,
}

impl<T> Display for ActionUi<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.label)?;
		for field in &self.fields {
			write!(f, "\n  {}", field)?;
		}
		Ok(())
	}
}

pub trait IntoActionUi<T> {
	fn into_action_ui(
		self,
		on_change: impl 'static + Clone + Fn(&Self),
	) -> ActionUi<T>;
}

// #[derive(Display)]
pub enum PropUi {
	// Group(GroupProp),
	Text(TextProp),
	Checkbox(CheckboxProp),
}


impl Display for PropUi {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PropUi::Text(val) => write!(f, "{val}"),
			PropUi::Checkbox(val) => write!(f, "{val}"),
		}
	}
}

pub struct GroupProp {
	pub label: String,
	pub children: Vec<PropUi>,
}

pub struct CheckboxProp {
	pub label: String,
	// pub get_value: Box<dyn Fn() -> bool>,
	pub get_cb: Box<dyn Fn() -> bool>,
	pub set_cb: Box<dyn Fn(bool)>,
}

impl CheckboxProp {
	pub fn get(&self) -> bool { (self.get_cb)() }
	pub fn set(&self, value: bool) { (self.set_cb)(value); }
}
impl Display for CheckboxProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.label, (self.get_cb)())
	}
}

pub struct TextProp {
	pub label: String,
	pub get_cb: Box<dyn Fn() -> String>,
	pub set_cb: Box<dyn Fn(&str)>,
}

impl TextProp {
	pub fn get(&self) -> String { (self.get_cb)() }
	pub fn set(&self, value: &str) { (self.set_cb)(value); }
}

impl Display for TextProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.label, (self.get_cb)())
	}
}

// impl Into<PropUi> for GroupProp {
// 	fn into(self) -> PropUi { PropUi::Group(self) }
// }

impl Into<PropUi> for TextProp {
	fn into(self) -> PropUi { PropUi::Text(self) }
}

impl Into<PropUi> for CheckboxProp {
	fn into(self) -> PropUi { PropUi::Checkbox(self) }
}
// impl ActionUi for CheckboxActionUi {
// 	fn get_type(&self) -> ActionUiType { ActionUiType::Checkbox }
// 	fn as_any(&self) -> &dyn Any { self }
// 	fn as_any_mut(&mut self) -> &mut dyn Any { self }
// }

// pub struct DropdownActionUi {
// 	pub name: String,
// 	pub value: String,
// 	pub options: Vec<String>,
// }

// impl ActionUi for DropdownActionUi {
// 	fn get_type(&self) -> ActionUiType { ActionUiType::Dropdown }
// 	fn as_any(&self) -> &dyn Any { self }
// 	fn as_any_mut(&mut self) -> &mut dyn Any { self }
// }

// pub struct NumberActionUi {
// 	pub name: String,
// 	pub value: f32,
// 	pub min: f32,
// 	pub max: f32,
// 	pub step: f32,
// }

// impl ActionUi for NumberActionUi {
// 	fn get_type(&self) -> ActionUiType { ActionUiType::Number }
// 	fn as_any(&self) -> &dyn Any { self }
// 	fn as_any_mut(&mut self) -> &mut dyn Any { self }
// }

// pub struct SliderActionUi {
// 	pub name: String,
// 	pub value: f32,
// 	pub min: f32,
// 	pub max: f32,
// 	pub step: f32,
// }

// impl ActionUi for SliderActionUi {
// 	fn get_type(&self) -> ActionUiType { ActionUiType::Slider }
// 	fn as_any(&self) -> &dyn Any { self }
// 	fn as_any_mut(&mut self) -> &mut dyn Any { self }
// }
