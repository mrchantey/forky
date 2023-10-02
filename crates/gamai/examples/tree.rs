#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy_ecs::prelude::*;
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;

type MyTree = gamai::tree!{
	<first_valid_edge edge=edge_always_pass>
	<empty_node edge=edge_always_fail/>
	<empty_node edge=edge_always_pass/>
	</first_valid_edge>
};
type MyOtherTree = gamai::tree!(<empty_node edge=edge_always_fail/>);

fn bizz(){}

fn main() {
	let _baz = bizz.clone();
	// bizz.add_node_system::<MyTree>();
	let mut world = World::new();
	world.spawn(MyTree::default());
	let mut schedule = Schedule::new();
	MyTree::add_systems(&mut schedule);
	// println!("{:?}", out);

	let a = foo(MyStruct(0));
	let b = foo(MyOtherStruct(3));
	// a.
	// a.
}


fn foo<T: Tree>(a: T)-> impl AiNode {
	a.build()
	// T::build()
	// let mut b = MyOtherStruct::build();
	// b = a;
}

trait Tree {
	fn build(&self) -> impl AiNode;
}


struct MyStruct(usize);

impl Tree for MyStruct {
	fn build(&self) -> impl AiNode { MyTree::default() }
}

struct MyOtherStruct(u32);
impl Tree for MyOtherStruct {
	fn build(&self) -> impl AiNode { MyOtherTree::default() }
}


/*


fn my_node()->impl IntoNode{


}


*/