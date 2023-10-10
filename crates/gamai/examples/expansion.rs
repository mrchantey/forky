// Recursive expansion of define_node! macro
// ==========================================
#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
// use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use gamai::*;


fn main() {
	// let _tree = || {
	// type Child = Node0<0, 0, 0, 0, DefaultAttributes>;
	// let _a = Child::default();
	// type Parent = Node1<0usize, 0, 0, 0, DefaultAttributes, Child>;
	// let a = Parent::default();
	// let b = Parent::into_depth();
	// type A = Parent::NewDepth;
	// let _a = Parent::into_default();
	// type Parent2 = Node1<0usize, 0, 0, 0, 0, DefaultAttributes, Parent>;
	// let a = Parent2::default();
	// assert_eq!(NodeInspector::graph_depth(&a), 0);
	// assert_eq!(NodeInspector::graph_depth(&a.child0), 1);

	// let child = || Node0::<0, 0, 0, 0, 0, _>::new(Attributes::default());
	// let _parent = Node1::<0usize, 0, 0, 0, 0, _, _>::new(
	// 	Attributes::default(),
	// 	move || child,
	// );
	// let parent2 = Node1::<0usize, 0, 0, 0, 0, _, _>::new(
	// 	Attributes::default(),
	// 	move || parent,
	// );
	// parent2
	// };
}

impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode,
	> Default for Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
{
	fn default() -> Self {
		Self {
			attributes: Default::default(),
			child0: Default::default(),
		}
	}
}

pub struct Node1<
	const GRAPH_ID: usize,
	const GRAPH_DEPTH: usize,
	const CHILD_INDEX: usize,
	const NODE_ID: usize,
	Attr: IntoAttributes,
	Child0: AiNode,
> {
	attributes: Attr,
	child0: Child0,
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode,
	> Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
{
	pub fn new(
		attributes: Attr,
		child0: impl IntoChildNode<GRAPH_ID, { GRAPH_DEPTH + 1 }, 0usize, 0, Child0>,
	) -> Self {
		Self {
			attributes,
			child0: child0.into_child_node(),
		}
	}
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode,
	> IntoNodeId
	for Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
{
	const GRAPH_ID: usize = GRAPH_ID;
	const GRAPH_DEPTH: usize = GRAPH_DEPTH;
	const CHILD_INDEX: usize = CHILD_INDEX;
	const NODE_ID: usize = NODE_ID;
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode,
	> AiNode for Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
where
	[(); GRAPH_DEPTH - 1]:,
{
	type ChildQuery = (
		Entity,
		(
			&'static mut DerefEdgeState<Child0>,
			Option<&'static mut DerefNodeState<Child0>>,
		),
	);
	#[allow(unused_parens)]
	type ChildBundle = ((AiBundle<Child0>,));
	fn add_systems(self, schedule: &mut Schedule) {
		if GRAPH_DEPTH != 0 {
			schedule.configure_set(
				BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
					.after(NodeSet::<GRAPH_ID, { GRAPH_DEPTH - 1 }>),
			);
			schedule.configure_set(
				NodeSet::<GRAPH_ID, GRAPH_DEPTH>
					.after(BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>),
			);
		} else {
			schedule.configure_set(
				BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
					.before(NodeSet::<GRAPH_ID, GRAPH_DEPTH>),
			);
		}
		self.child0.add_systems(schedule);
	}
	fn entity<'a>(
		val: &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>,
	) -> Entity {
		val.0
	}
	fn children<'a>(
		(_entity,(_edge0,node0,)): <Self::ChildQuery as bevy_ecs::query::WorldQuery> ::Item<'a>,
	) -> Vec<ChildState<'a>> {
		let _node0 = if let Some(val) = node0 {
			Some(val.into_inner() as DerefNode<'_>)
		} else {
			None
		};
		vec![]
	}
	fn get_child(&self, index: usize) -> &dyn NodeInspector {
		match index {
			0usize => &self.child0,
			_ => {
				panic!();
			}
		}
	}
	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector> {
		match index {
			0usize => Box::new(self.child0),
			_ => {
				panic!();
			}
		}
	}
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode,
	> IntoRootNode
	for Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
where
	[(); GRAPH_DEPTH - 1]:,
{
	type Out = Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>;
	fn into_root_node(self) -> Self::Out { self }
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		const NEW_GRAPH_ID: usize,
		const NEW_GRAPH_DEPTH: usize,
		const NEW_CHILD_INDEX: usize,
		const NEW_NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode
			+ IntoChildNode<
				NEW_GRAPH_ID,
				{ NEW_GRAPH_DEPTH + 1 },
				0usize,
				0,
				NewChild0,
			>,
		NewChild0: AiNode,
	>
	IntoChildNode<
		NEW_GRAPH_ID,
		NEW_GRAPH_DEPTH,
		NEW_CHILD_INDEX,
		NEW_NODE_ID,
		Node1<
			NEW_GRAPH_ID,
			NEW_GRAPH_DEPTH,
			NEW_CHILD_INDEX,
			NEW_NODE_ID,
			Attr,
			NewChild0,
		>,
	> for Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
{
	fn into_child_node(
		self,
	) -> Node1<
		NEW_GRAPH_ID,
		NEW_GRAPH_DEPTH,
		NEW_CHILD_INDEX,
		NEW_NODE_ID,
		Attr,
		NewChild0,
	> {
		Node1 {
			attributes: self.attributes,
			child0: self.child0.into_child_node(),
		}
	}
}



impl<
		const NEW_DEPTH: usize,
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
		Child0: AiNode + IntoDepth<{ NEW_DEPTH + 1 }, NewChild0>,
		NewChild0: AiNode,
	>
	IntoDepth<
		NEW_DEPTH,
		Node1<GRAPH_ID, NEW_DEPTH, CHILD_INDEX, NODE_ID, Attr, NewChild0>,
	> for Node1<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr, Child0>
{
}


impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
	> Default for Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
{
	fn default() -> Self {
		Self {
			attributes: Default::default(),
		}
	}
}



pub struct Node0<
	const GRAPH_ID: usize,
	const GRAPH_DEPTH: usize,
	const CHILD_INDEX: usize,
	const NODE_ID: usize,
	Attr: IntoAttributes,
> {
	attributes: Attr,
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
	> Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
{
	pub fn new(attributes: Attr) -> Self { Self { attributes } }
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
	> IntoNodeId for Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
{
	const GRAPH_ID: usize = GRAPH_ID;
	const GRAPH_DEPTH: usize = GRAPH_DEPTH;
	const CHILD_INDEX: usize = CHILD_INDEX;
	const NODE_ID: usize = NODE_ID;
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
	> AiNode for Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
where
	[(); GRAPH_DEPTH - 1]:,
{
	type ChildQuery = (Entity,);
	#[allow(unused_parens)]
	type ChildBundle = ();
	fn add_systems(self, schedule: &mut Schedule) {
		if GRAPH_DEPTH != 0 {
			schedule.configure_set(
				BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
					.after(NodeSet::<GRAPH_ID, { GRAPH_DEPTH - 1 }>),
			);
			schedule.configure_set(
				NodeSet::<GRAPH_ID, GRAPH_DEPTH>
					.after(BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>),
			);
		} else {
			schedule.configure_set(
				BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
					.before(NodeSet::<GRAPH_ID, GRAPH_DEPTH>),
			);
		}
	}
	fn entity<'a>(
		val: &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>,
	) -> Entity {
		val.0
	}
	fn children<'a>(
		(_entity,): <Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a>> {
		vec![]
	}
	fn get_child(&self, index: usize) -> &dyn NodeInspector {
		match index {
			_ => {
				panic!()
			}
		}
	}
	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector> {
		match index {
			_ => {
				panic!()
			}
		}
	}
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
	> IntoRootNode for Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
where
	[(); GRAPH_DEPTH - 1]:,
{
	type Out = Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>;
	fn into_root_node(self) -> Self::Out { self }
}
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		const NEW_GRAPH_ID: usize,
		const NEW_GRAPH_DEPTH: usize,
		const NEW_CHILD_INDEX: usize,
		const NEW_NODE_ID: usize,
		Attr: IntoAttributes,
	>
	IntoChildNode<
		NEW_GRAPH_ID,
		NEW_GRAPH_DEPTH,
		NEW_CHILD_INDEX,
		NEW_NODE_ID,
		Node0<
			NEW_GRAPH_ID,
			NEW_GRAPH_DEPTH,
			NEW_CHILD_INDEX,
			NEW_NODE_ID,
			Attr,
		>,
	> for Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
{
	fn into_child_node(
		self,
	) -> Node0<NEW_GRAPH_ID, NEW_GRAPH_DEPTH, NEW_CHILD_INDEX, NEW_NODE_ID, Attr>
	{
		Node0 {
			attributes: self.attributes,
		}
	}
}

impl<
		const NEW_DEPTH: usize,
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		Attr: IntoAttributes,
	> IntoDepth<NEW_DEPTH, Node0<GRAPH_ID, NEW_DEPTH, CHILD_INDEX, NODE_ID, Attr>>
	for Node0<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, Attr>
{
}
