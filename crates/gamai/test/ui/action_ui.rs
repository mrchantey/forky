use gamai::prelude::*;
use std::any::Any;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use sweet::*;


#[derive(Debug)]
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
		ActionUi {
			label: "My Action".to_string(),
			value: this.clone(),
			fields: vec![
				{
					let this2 = this.clone();
					let this3 = this.clone();
					let on_change = on_change.clone();
					TextProp {
						label: heck::AsTitleCase("value_str").to_string(),
						get_cb: Box::new(move || {
							this2.borrow().value_str.clone()
						}),
						set_cb: Box::new(move |value| {
							this3.borrow_mut().value_str = value.to_string();
							on_change(&*this3.borrow());
						}),
					}
					.into()
				},
				{
					let this2 = this.clone();
					let this3 = this.clone();
					let on_change = on_change.clone();
					CheckboxProp {
						label: heck::AsTitleCase("value_bool").to_string(),
						get_cb: Box::new(move || {
							this2.borrow().value_bool.clone()
						}),
						set_cb: Box::new(move |value| {
							this3.borrow_mut().value_bool = value;
							on_change(&*this3.borrow());
						}),
					}
					.into()
				},
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
		PropUi::Text(text) => {
			text.set("hello2");
			expect(&text.label).to_be(&"Value Str".to_string())?;
			expect(&text.get()).to_be(&"hello2".to_string())?;
		}
		_ => panic!("expected text"),
	}
	Ok(())
}
