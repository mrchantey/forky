use sweet::*;

enum Foo {
	A,
	B,
	C(u32),
}

struct Bar {
	pub foo: Foo,
}

struct Bazz {
	pub bar: Bar,
}

/*

something like this


score: Dropdown<Pass,Weight,Fail>
if Weight is selected, then show a slider



| Some Action  																|
| Bazz           															|
| 	Bar           														|
| 		Foo  (dropdown)     										|
|				in the case of Foo::C, show a slider	|
*/


#[sweet_test]
pub fn works() -> Result<()> {
	let a = Bazz {
		bar: Bar { foo: Foo::A },
	};

	// expect(true).to_be_false()?;

	Ok(())
}
