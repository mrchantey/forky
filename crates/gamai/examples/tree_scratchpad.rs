#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	associated_type_bounds,
	// const_evaluatable_checked,
	// const_generics,
	// generic_const_exprs
)]
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;
// use std::marker::PhantomData;

fn main() {}

fn my_node_inner<
	const CHILD_INDEX: usize,
	const GRAPH_ID: usize,
	const PARENT_DEPTH: usize,
	const GRANDPARENT_DEPTH: usize,
	Parent,
>() -> impl AiNode
where
	Parent: IntoNodeId<
		GRAPH_ID = { GRAPH_ID },
		GRAPH_DEPTH = { PARENT_DEPTH },
		PARENT_DEPTH = { GRANDPARENT_DEPTH },
	>,
{
	Node0::<CHILD_INDEX, Parent, _, _, _, _>::new(|| || {}, || || {})
}

#[allow(non_camel_case_types)]
struct my_node;

impl IntoTree for my_node {
	fn build_root() -> impl AiNode {
		my_node_inner::<0, 0, 0, 0, RootParent<0>>() //here the graph id would be set in the macro
	}
	fn build_child<
		const CHILD_INDEX: usize,
		const GRAPH_ID: usize,
		const PARENT_DEPTH: usize,
		const GRANDPARENT_DEPTH: usize,
		Parent,
	>() -> impl AiNode
	where
		Parent: IntoNodeId<
			GRAPH_ID = { GRAPH_ID },
			GRAPH_DEPTH = { PARENT_DEPTH },
			PARENT_DEPTH = { GRANDPARENT_DEPTH },
		>,
	{
		my_node_inner::<
			CHILD_INDEX,
			GRAPH_ID,
			PARENT_DEPTH,
			GRANDPARENT_DEPTH,
			Parent,
		>()
	}
}
