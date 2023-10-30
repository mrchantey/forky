#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;

#[action]
fn my_system<N: AiNode>() {}

fn main() {
	let tree1 = || tree! {<my_system/>};
	let _bundle = TreeBundle::new(tree1);
	let _bundle = TreeBundle::new(tree1);
	let _plugin = TreePlugin::new(tree1);
	let _plugin = TreePlugin::new(tree1);

	let a = foobar().foo();
	println!("{}", a);
}



trait MyTrait {
	fn foo(&self) -> u32;
}
fn foobar() -> impl MyTrait {
	pub struct MyStruct {
		pub val: u32,
	}
	impl MyTrait for MyStruct {
		fn foo(&self) -> u32 { self.val }
	}
	MyStruct { val: 3 }
}
