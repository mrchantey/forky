use super::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub trait IntoRootNode {
	type Out: AiNode;
	fn into_root_node(self) -> Self::Out;
}

// implement for factories
impl<F, T> IntoRootNode for F
where
	T: IntoRootNode,
	F: FnOnce() -> T,
{
	type Out = T::Out;
	fn into_root_node(self) -> Self::Out { self().into_root_node() }
}

pub trait IntoChildNode<const CHILD_INDEX: usize, Parent: IntoNodeId>:
	'static + Send + Sync + Sized
{
	type Out: AiNode;
	fn into_child_node(self) -> Self::Out;
}

// implement for factories
impl<const CHILD_INDEX: usize, Parent: IntoNodeId, F, T>
	IntoChildNode<CHILD_INDEX, Parent> for F
where
	T: IntoChildNode<CHILD_INDEX, Parent>,
	F: 'static + Send + Sync + FnOnce() -> T,
{
	type Out = T::Out;
	fn into_child_node(self) -> Self::Out { self().into_child_node() }
}


pub trait IntoNode: 'static + Send + Sync + Copy {
	fn bundle(self) -> impl Bundle {
		AiBundle::new(move || self.get_into_root_node())
	}
	fn plugin(self) -> impl Plugin {
		AiPlugin::new(move || self.get_into_root_node())
	}

	fn get_into_root_node(self) -> impl IntoRootNode;

	fn get_into_child_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
		self,
	) -> impl IntoChildNode<CHILD_INDEX, Parent>;
}

pub trait Tree: 'static + Send + Sync + Copy {
	fn build(self) -> impl IntoNode;
}


impl<T> IntoNode for T
where
	T: Tree,
{
	fn get_into_root_node(self) -> impl IntoRootNode {
		self.build().get_into_root_node()
	}

	fn get_into_child_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
		self,
	) -> impl IntoChildNode<CHILD_INDEX, Parent> {
		self.build().get_into_child_node()
	}
}

// impl<F, T> IntoNode for F
// where
// 	F: FnOnce() -> T,
// 	T: IntoNode,
// {
// 	fn get_into_root_node(self) -> impl IntoRootNode {
// 		self().get_into_root_node()
// 	}

// 	fn get_into_child_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
// 		self,
// 	) -> impl IntoChildNode<CHILD_INDEX, Parent> {
// 		self().get_into_child_node()
// 	}
// }


// pub trait IntoNode<Node: AiNode>: 'static + Send + Sync + Sized {
// 	fn into_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(&self) -> Node;
// pub trait IntoNode: 'static + Send + Sync + Sized {
// 	type Out<const CHILD_INDEX:usize,Parent:IntoNodeId>:AiNode;
// 	fn into_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
// 		&self,
// 	) -> Self::Out<CHILD_INDEX, Parent>;
// }

// pub trait IntoChildNode {
// 	fn into_child_node<
// 		const CHILD_INDEX: usize,
// 		const GRAPH_ID: usize,
// 		const PARENT_DEPTH: usize,
// 		const GRANDPARENT_DEPTH: usize,
// 		Parent,
// 	>(
// 		self,
// 	) -> impl AiNode
// 	where
// 		Parent: IntoNodeId<
// 			GRAPH_ID = { GRAPH_ID },
// 			GRAPH_DEPTH = { PARENT_DEPTH },
// 			PARENT_DEPTH = { GRANDPARENT_DEPTH },
// 		>;
// }

// pub trait IntoTree: IntoRootNode + IntoChildNode {}
// impl<T> IntoTree for T where T: IntoRootNode + IntoChildNode {}

// impl<F, T> IntoChildNode for F
// where
// 	T: IntoChildNode,
// 	F: Fn() -> T,
// {
// 	fn into_child_node<
// 		const CHILD_INDEX: usize,
// 		const GRAPH_ID: usize,
// 		const PARENT_DEPTH: usize,
// 		const GRANDPARENT_DEPTH: usize,
// 		Parent,
// 	>(
// 		self,
// 	) -> impl AiNode
// 	where
// 		Parent: IntoNodeId<
// 			GRAPH_ID = { GRAPH_ID },
// 			GRAPH_DEPTH = { PARENT_DEPTH },
// 			PARENT_DEPTH = { GRANDPARENT_DEPTH },
// 		>,
// 	{
// 		self().into_child_node::<CHILD_INDEX,GRAPH_ID,PARENT_DEPTH,GRANDPARENT_DEPTH,Parent>()
// 	}
// }

// impl<const CHILD_INDEX: usize, Parent: IntoNodeId, F, T>
// 	IntoNode<CHILD_INDEX, Parent> for F
// where
// 	T: IntoNode<CHILD_INDEX, Parent>,
// 	F: 'static + Send + Sync + Fn() -> T,
// {
// 	type Out = T::Out;
// 	fn into_node(self) -> Self::Out { self().into_node() }
// }

// fn node_state(self, world: &World, entity: Entity) -> Option<NodeState> {
// 	world
// 		.entity(entity)
// 		.get::<DerefNodeState<Node>>()
// 		.map(|s| **s)
// }
// fn edge_state(self, world: &World, entity: Entity) -> Option<EdgeState> {
// 	world
// 		.entity(entity)
// 		.get::<DerefEdgeState<Node>>()
// 		.map(|s| **s)
// }

// impl<F, Node> IntoNode<Node> for F
// where
// 	Node: AiNode,
// 	F: 'static + Send + Sync + Fn() -> Node,
// {
// }
