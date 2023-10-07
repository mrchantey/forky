#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
#![recursion_limit = "256"]
fn main() {}


// trait Node {
// 	const DEPTH: usize;
// 	fn into_child() -> impl Node<DEPTH = { Self::DEPTH + 1 }>
// 	where
// 		[(); Self::DEPTH + 1]:;
// }

// struct Foo<const DEPTH: usize>;

// impl<const DEPTH: usize> Node for Foo<DEPTH> {
// 	const DEPTH: usize = DEPTH;

// 	// fn into_child() -> impl Node<DEPTH = { Self::DEPTH + 1 }>
// 	fn into_child() -> impl Node<DEPTH = { <Foo<{ DEPTH + 1 }> as Node>::DEPTH }>
// 	where
// 		[(); Self::DEPTH + 1]:,
// 		// <Foo<{ DEPTH + 1 }> as Node>::DEPTH
// 	{
// 		Foo
// 		// Foo::<{ DEPTH + 1 }>
// 		// Foo::<{ DEPTH + 1 }>
// 	}
// }
