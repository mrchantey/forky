#![feature(
	generic_const_exprs,
	inherent_associated_types,
	return_position_impl_trait_in_trait
)]
#![allow(incomplete_features)]

struct Node<const DEPTH: usize>;

trait Foo {}
trait Bar {
	fn into_foo(self) -> impl Foo;
	// fn into_foo(self) -> impl Foo + Bar;
}

impl<const DEPTH: usize> Foo for Node<DEPTH> {}
impl<const DEPTH: usize> Bar for Node<DEPTH>
where
	[(); DEPTH + 1]:,
	// Node<{ DEPTH + 1 }>: Foo + Bar,
{
	fn into_foo(self) -> impl Foo { self.into_child() }
}

impl<const DEPTH: usize> Node<DEPTH> {
	pub fn get_depth(&self) -> usize { DEPTH }
	pub fn into_child(self) -> Node<{ DEPTH + 1 }> { Node::<{ DEPTH + 1 }> }
}

// trait IntoChild<Out> {
// 	fn into_child(self) -> Out;
// }

// impl<const DEPTH: usize> IntoChild<Node<{ DEPTH + 1 }>> for Node<DEPTH>
// {
// 	fn into_child(self) -> Node<{ DEPTH + 1 }> { Node::<{ DEPTH + 1 }> }
// }

// fn foo()

// fn foo<T>(_val: impl IntoChild<T>) {}

fn main() {
	let parent = Node::<0>;
	assert_eq!(parent.get_depth(), 0);
	let _a = parent.into_foo();
	// let child = foo(parent);
	// let child = parent.into_child();
	// assert_eq!(child.get_depth(), 1);

	// const MAX_USIZE_64BIT: usize = usize::MAX;
	// OK
	// let parent = Node::<MAX_USIZE_64BIT>;
	// ERROR: attempt to compute `0_usize - 1_usize`, which would overflow
	// let child = parent.into_child();
}


trait IntoChildOuter {
	type Out: IntoChildOuter;
	fn into_child(self) -> Self::Out;
}


// impl<In,Out> IntoChildOuter for In
// where
// 	In: IntoChild<Out>,
// 	// Out: IntoChildOuter,
// {
// 	type Out = Out;
// 	fn into_child(self) -> Self::Out { self.into_child() }
// }
