use bevy_reflect::ParsedPath;
use bevy_reflect::Reflect;
use bevy_reflect::Struct;
use gamai::prelude::*;
use std::any::Any;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use sweet::*;

#[derive(Debug, Reflect)]
pub struct Wrapper(pub u32);

#[derive(Debug, Reflect)]
pub struct MyAction {
	pub val_str: String,
	pub val_bool: bool,
	pub val_score: Score,
	pub val_wrapper: Wrapper,
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
					"val_str".to_string(),
					ParsedPath::parse("val_str").unwrap(),
					Some(on_change.clone()),
				)),
				bool::into_field_ui(FieldReflect::new(
					this.clone(),
					"val_bool".to_string(),
					ParsedPath::parse("val_bool").unwrap(),
					Some(on_change.clone()),
				)),
				Score::into_field_ui(FieldReflect::new(
					this.clone(),
					"val_score".to_string(),
					ParsedPath::parse("val_score").unwrap(),
					Some(on_change.clone()),
				)),
				// Wrapper::into_field_ui(FieldReflect::new(
				// 	this.clone(),
				// 	"val_wrapper".to_string(),
				// 	Some(on_change.clone()),
				// )),
			],
		}
	}
}


#[sweet_test]
pub fn works() -> Result<()> {
	let action = MyAction {
		val_str: "hello".to_string(),
		val_bool: true,
		val_score: Score::Pass,
		val_wrapper: Wrapper(42),
	};
	let ui = action.into_action_ui(|_| {});

	expect(ui.label).to_be("My Action".to_string())?;

	match &ui.fields[0] {
		FieldUi::Text(text) => {
			text.set("hello2".to_string());
			expect(&text.display_name).to_be(&"Val Str".to_string())?;
			expect(&text.get()).to_be(&"hello2".to_string())?;
		}
		_ => panic!("expected text"),
	}
	match &ui.fields[2] {
		FieldUi::Select(select) => {
			expect(&select.options()[1]).to_be(&"Weight".to_string())?;
			expect(&select.options()[select.get_index()])
				.to_be(&"Pass".to_string())?;
			select.set_index(0);
			expect(select.get_index()).to_be(0)?;
		}
		_ => panic!("expected text"),
	}
	Ok(())
}
