// Recursive expansion of define_node! macro
// ==========================================
#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs,
	associated_type_defaults
)]
#![allow(incomplete_features)]
// use bevy_app::prelude::*;
// use bevy_ecs::prelude::*;
// use gamai::*;

// use bevy_ecs::schedule::Schedule;
// use bevy_ecs::schedule::SystemSet;
use bevy_ecs::prelude::*;
use gamai::*;
use std::marker::PhantomData;

fn main() {
	type Root = TreePathRoot<0>;
	let a = Node0::<Root, _>::new(DefaultAttributes::default());
	let b = Node1::<Root, _, _>::new(DefaultAttributes::default(), || a)
		.into_root();

	let c = b.clone().into_child::<TreePathRoot<3>>();
	assert_eq!(b.graph_id(), 0);
	assert_eq!(c.graph_id(), 3);
	assert_eq!(b.child(0).graph_depth(), 2);
	// let bundle1 = b.clone().bundle();
	// assert!(bundle1 != bundle2);

	// let b = b.into_child::<TreePathRoot<3>>();
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
struct Node0<Path: TreePath, Node: IntoNodeSystem> {
	pub node: Node,
	pub phantom: PhantomData<Path>,
}
impl<Path: TreePath, Node: IntoNodeSystem> Node0<Path, Node> {
	pub fn new(node: Node) -> Self {
		Self {
			node,
			phantom: PhantomData,
		}
	}
}

impl<Path: TreePath, N: IntoNodeSystem> TreePath for Node0<Path, N> {
	type Parent = Path::Parent;
	const CHILD_INDEX: usize = Path::CHILD_INDEX;
}

impl<Path: TreePath, N: IntoNodeSystem> AiNode for Node0<Path, N> {
	type ChildQuery = (Entity,);
	#[allow(unused_parens)]
	type ChildBundle = ();
	fn add_systems(self, schedule: &mut Schedule) {
		Self::configure_sets(schedule);
		schedule.add_systems(self.node.into_node_system_configs::<Self>());
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

	fn into_child<NewPath: TreePath>(self) -> impl AiNode {
		Node0::<NewPath, N>::new(self.node)
	}

	fn get_child(&self, _index: usize) -> &dyn NodeInspector {
		panic!("child index out of bounds")
	}

	fn get_child_owned(self, _index: usize) -> Box<dyn NodeInspector> {
		panic!("child index out of bounds")
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node1<Path: TreePath, Node: IntoNodeSystem, Child0: AiNode> {
	pub node: Node,
	pub child0: Child0,
	pub phantom: PhantomData<Path>,
}
impl<Path: TreePath, N: IntoNodeSystem, Child0: AiNode> Node1<Path, N, Child0> {
	pub fn new<IntoChild0Marker>(
		node: N,
		child0: impl IntoNode<IntoChild0Marker, Out = Child0>,
	) -> Self {
		Self {
			node,
			child0: child0.into_node(),
			phantom: PhantomData,
		}
	}
}

impl<Path: TreePath, System: IntoNodeSystem, Child0: AiNode> TreePath
	for Node1<Path, System, Child0>
{
	type Parent = Path::Parent;
	const CHILD_INDEX: usize = Path::CHILD_INDEX;
}

impl<Path: TreePath, N: IntoNodeSystem, Child0: AiNode> AiNode
	for Node1<Path, N, Child0>
{
	type ChildQuery = (
		Entity,
		(
			&'static mut DerefEdgeState<Child0>,
			Option<&'static mut DerefNodeState<Child0>>,
		),
	);
	#[allow(unused_parens)]
	type ChildBundle = ();
	fn add_systems(self, schedule: &mut Schedule) {
		Self::configure_sets(schedule);
		schedule.add_systems(self.node.into_node_system_configs::<Self>());
		self.child0.add_systems(schedule);
	}
	fn entity<'a>(
		val: &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>,
	) -> Entity {
		val.0
	}
	fn children<'a>(
		(_entity,_child0): <Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a>> {
		vec![]
	}

	fn into_child<NewPath: TreePath>(self) -> impl AiNode {
		Node1::<NewPath, _, _>::new(
			self.node,
			// self.child0
			|| self.child0.into_child::<TreePathSegment<0, Self>>(),
		)
	}

	type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;

	fn get_child(&self, index: usize) -> &dyn NodeInspector {
		match index {
			0 => &self.child0,
			_ => panic!("child index out of bounds"),
		}
	}

	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector> {
		match index {
			0 => Box::new(self.child0),
			_ => panic!("child index out of bounds"),
		}
	}
}


// impl<Path: TreePath, System: IntoNodeSystem,Child0:AiNode> IntoChildPath
// 	for Node1<Path, System,Child0>
// {
// 	type Out<NewPath: TreePath> = Node1<NewPath, System,Child0>;
// 	fn into_child<NewPath: TreePath>(self) -> Self::Out<NewPath> {
// 		Self::Out::new(self.node)
// 	}
// }


// impl<Path: TreePath, N: IntoNodeSystem, Child0: AiNode> IntoNodeSystem
// 	for Node1<Path, N, Child0>
// {
// 	fn into_node_system_configs<Node: gamai::AiNode>(
// 		self,
// 	) -> bevy_ecs::schedule::SystemConfigs {
// 		(
// 			self.node.into_node_system_configs::<Node>(),
// 			self.child0.into_node_system_configs::<Node>(),
// 		)
// 			.into_configs()
// 	}
// }


// impl<Path: TreePath, N: IntoNodeSystem, Child0: AiNode> IntoChildPath
// 	for Node1<Path, N, Child0>
// {
//     fn into_child<Parent: TreePath>(self) -> impl IntoChildPath {
// 		Node1::<Parent, N, Child0>
//     }
// }
