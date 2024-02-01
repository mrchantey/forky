use gamai::prelude::*;
use std::cell::RefCell;
use sweet::*;


#[derive(Clone, FieldUi)]
struct MyAction {
	#[slider(min = 0, max = 100, step = 1)]
	pub health: u32,
	pub score: Score,
	pub nested: NestedAction,
}

#[derive(Clone, FieldUi)]
struct NestedAction {
	pub nested_field: u32,
}


fn setup() -> FieldUiRoot<MyAction> {
	FieldUiRoot::new(MyAction {
		health: 100,
		score: Score::Pass,
		nested: NestedAction { nested_field: 0 },
	})
}


#[sweet_test]
pub fn sets_value() -> Result<()> {
	let root = setup();

	if let FieldUi::Group(group) = root.get_ui() {
		if let FieldUi::SliderU32(slider) = &group.children[0] {
			slider.set(50);
		} else {
			anyhow::bail!("Expected FieldUi");
		}
	} else {
		anyhow::bail!("Expected FieldUi");
	}

	expect(root.borrow().health).to_be(50)?;

	Ok(())
}


#[sweet_test]
pub fn sets_nested_value() -> Result<()> {
	let root = setup();

	if let FieldUi::Group(group) = root.get_ui() {
		if let FieldUi::Group(nested_group) = &group.children[2] {
			if let FieldUi::NumberU32(nested_field) = &nested_group.children[0]
			{
				nested_field.set(50);
			} else {
				anyhow::bail!("Expected FieldUi");
			}
		} else {
			anyhow::bail!("Expected FieldUi");
		}
	} else {
		anyhow::bail!("Expected FieldUi");
	}

	expect(root.borrow().nested.nested_field).to_be(50)?;

	Ok(())
}


#[sweet_test]
pub fn recalculates_ui() -> Result<()> {
	let root = setup().with_on_ui_change(|ui| {
		println!("UI Refresh: \n{}", ui.into_string_tree());
	});

	if let FieldUi::Group(group) = root.get_ui() {
		if let FieldUi::Select(select) = &group.children[1] {
			select.set(Score::Weight(0.1))?;
		} else {
			anyhow::bail!("Expected Select");
		}
	// if let FieldUi::Group(group) = &group.children[1] {
	// 	// select.set(Score::Weight(0.1));
	// } else {
	// 	anyhow::bail!("Expected Group");
	// }
	} else {
		anyhow::bail!("Expected FieldUi");
	}

	expect(root.borrow().score).to_be(Score::Weight(0.1))?;

	Ok(())
}
