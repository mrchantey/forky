use bevy_reflect::Reflect;
use gamai::prelude::*;
use std::any::Any;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use sweet::*;


#[derive(Debug, Reflect)]
pub struct MyAction {
	pub value_str: String,
	pub value_bool: bool,
}

impl IntoActionUi<Self> for MyAction {
	fn into_action_ui(
		self,
		on_change: impl 'static + Clone + Fn(&Self),
	) -> ActionUi<Self> {
		let this = Rc::new(RefCell::new(self));
		let on_change = Box::new(on_change);
		ActionUi {
			label: "My Action".to_string(),
			value: this.clone(),
			fields: vec![
				String::into_field_ui(FieldReflect::new(
					this.clone(),
					"value_str".to_string(),
					Some(on_change.clone()),
				)),
				bool::into_field_ui(FieldReflect::new(
					this.clone(),
					"value_bool".to_string(),
					Some(on_change.clone()),
				)),
			],
		}
	}
}


#[sweet_test]
pub fn works() -> Result<()> {
	let action = MyAction {
		value_str: "hello".to_string(),
		value_bool: true,
	};
	let ui = action.into_action_ui(|_| {});

	expect(ui.label).to_be("My Action".to_string())?;

	match &ui.fields[0] {
		FieldUi::Text(text) => {
			text.set("hello2".to_string());
			expect(&text.display_name).to_be(&"Value Str".to_string())?;
			expect(&text.get()).to_be(&"hello2".to_string())?;
		}
		_ => panic!("expected text"),
	}
	Ok(())
}
