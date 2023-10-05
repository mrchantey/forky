#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	associated_type_bounds,
	// const_evaluatable_checked,
	// const_generics,
)]
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;
// use std::marker::PhantomData;

fn main() {
	let node = foo(into_tree());
	println!("{:?}", node.graph_id())
}

fn foo(a: impl IntoTree) -> impl AiNode { a.into_root_node() }


fn into_tree() -> impl IntoTree {
	// pub struct FOO;
	#[allow(non_camel_case_types)]
	struct my_node;

	impl IntoRootNode for my_node {
		fn into_root_node(&self) -> impl AiNode {
			const GRAPH_ID: usize = 0;
			Self.into_child_node::<0, GRAPH_ID, 0, 0, RootParent<GRAPH_ID>>()
		}
	}
	impl IntoChildNode for my_node {
		fn into_child_node<
			const CHILD_INDEX: usize,
			const GRAPH_ID: usize,
			const PARENT_DEPTH: usize,
			const GRANDPARENT_DEPTH: usize,
			Parent,
		>(
			self,
		) -> impl AiNode
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
	my_node
}

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
