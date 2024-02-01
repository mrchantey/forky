use gamai::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use strum_macros::Display;
use strum_macros::EnumIter;
use sweet::*;

#[derive(Clone)]
struct Bar {
	pub foo: Foo,
}

impl IntoFieldUi for Bar {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		GroupField::new(reflect.display_name.clone(), vec![Foo::into_field_ui(
			FieldReflect::new(
				"foo".to_string(),
				{
					let get_cb = reflect.clone_get_cb();
					move || get_cb().foo.clone()
				},
				{
					let get_cb = reflect.clone_get_cb();
					let set_cb = reflect.clone_set_cb();
					move |val| {
						let mut bar = get_cb();
						bar.foo = val;
						set_cb(bar);
					}
				},
			),
		)])
		.into()
	}
}

#[derive(Clone, Display, EnumIter)]
enum Foo {
	A,
	B,
	C(u32),
}

impl IntoFieldUi for Foo {
	fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
		let foo_select = SelectField::new(
			reflect.display_name.clone(),
			reflect.clone_get_cb(),
			reflect.clone_set_cb(),
		);
		let val = reflect.get();
		// let get = reflect.get_cb;
		// let val = foo.borrow();
		match val {
			Foo::A => foo_select.into(),
			Foo::B => foo_select.into(),
			Foo::C(_) => {
				let checked_get = {
					let get_cb = reflect.clone_get_cb();
					move || match get_cb() {
						Foo::C(x) => x,
						_ => panic!("Expected Foo::C"),
					}
				};
				let checked_set = {
					let get_cb = reflect.clone_get_cb();
					let set_cb = reflect.clone_set_cb();
					move |val| match get_cb() {
						Foo::C(mut x) => {
							x = val;
							set_cb(Foo::C(x));
						}
						_ => panic!("Expected Foo::C"),
					}
				};

				let field0 = u32::into_field_ui(FieldReflect::new(
					"Field 0".to_string(),
					checked_get,
					checked_set,
				));

				GroupField::new(reflect.display_name.clone(), vec![
					foo_select.into(),
					field0.into(),
				])
			}
			.into(),
		}
	}
}





#[sweet_test]
pub fn works() -> Result<()> {
	// let a = Bar { foo: Foo::A };
	let a = Bar { foo: Foo::C(23) };

	let root = a.into_field_ui_root();

	println!("{}", root.into_string_tree());

	// expect(true).to_be_false()?;

	Ok(())
}


// fn foobar() -> FieldUi {
// 	let foo = Foo::C(32);
// 	let foo = Rc::new(RefCell::new(foo));
// 	let foo_select = SelectField::new(
// 		"foo".to_string(),
// 		{
// 			let foo = foo.clone();
// 			move || foo.borrow().clone()
// 		},
// 		{
// 			let foo = foo.clone();
// 			move |val| *foo.borrow_mut() = val
// 		},
// 	);
// 	let val = foo.borrow();
// 	match *val {
// 		Foo::A => foo_select.into(),
// 		Foo::B => foo_select.into(),
// 		Foo::C(_) => GroupField::new("foo".to_string(), vec![
// 			foo_select.into(),
// 			SliderField::new(
// 				"foo".to_string(),
// 				{
// 					let foo = foo.clone();
// 					move || match *foo.borrow() {
// 						// getter
// 						Foo::C(x) => x,
// 						_ => panic!("Expected Foo::C"),
// 					}
// 				},
// 				{
// 					let foo = foo.clone();
// 					move |val| match *foo.borrow_mut() {
// 						// setter
// 						Foo::C(ref mut x) => *x = val,
// 						_ => panic!("Expected Foo::C"),
// 					}
// 				},
// 				0,
// 				100,
// 				1,
// 			)
// 			.into(),
// 		])
// 		.into(),
// 	}
// }
