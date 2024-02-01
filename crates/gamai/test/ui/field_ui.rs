// use gamai::prelude::*;
// use std::any::Any;
// use std::cell::Ref;
// use std::cell::RefCell;
// use std::fmt::Display;
// use std::rc::Rc;
// use sweet::*;

// #[derive(Debug, Clone, Reflect)]
// pub struct MyStruct {
// 	pub val_score: Score,
// }


// impl<ParentT: FieldParent> IntoFieldUi<ParentT, Self> for MyStruct {
// 	fn into_field_ui(reflect: FieldReflect<ParentT, Self>) -> FieldUi<ParentT> {
// 		let mut path = reflect.path.to_string();
// 		path.push_str("val_score");
// 		let path = ParsedPath::parse(&path).unwrap();
// 		GroupField {
// 			name: "MyStruct".to_string(),
// 			children: vec![],
// 			// children: vec![Score::into_field_ui(FieldReflect::new(
// 			// 	reflect.root.clone(),
// 			// 	"val_score".to_string(),
// 			// 	path,
// 			// 	reflect.on_change,
// 			// ))],
// 		}
// 		.into()
// 	}
// }


// // impl<Root: FieldParent> IntoFieldReflect<Root> for MyStruct {
// // 	fn into_field_reflect(
// // 		self,
// // 		root: Rc<RefCell<Root>>,
// // 		name: String,
// // 		path: ParsedPath,
// // 		on_change: Option<Box<dyn Fn(&Root)>>,
// // 	) -> FieldReflect<Root, Self> {
// // 		FieldReflect::new(root, name, path, on_change)
// // 	}
// // }


// #[sweet_test]
// pub fn works() -> Result<()> {
// 	let val = MyStruct {
// 		val_score: Score::Weight(0.5),
// 	};

// 	val.val_score.iter_fields().for_each(|field| {
// 		println!("field: {:?}", field.name());
// 	});

// 	let field_ui = MyStruct::into_field_ui(FieldReflect::new(
// 		Rc::new(RefCell::new(val)),
// 		"Foobar".to_string(),
// 		ParsedPath::parse("").unwrap(),
// 		None,
// 	));

// 	let val = field_ui.into_string_tree().to_string();
// 	println!("{}", val);

// 	// expect(true).to_be_false()?;

// 	Ok(())
// }
