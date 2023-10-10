use super::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub trait AiTree: 'static + Send + Sync + Copy {
	fn get_into_root_node(self) -> impl IntoRootNode;

	fn get_into_child_node<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		const PARENT_DEPTH: usize,
		Marker,
	>(
		self,
	) -> impl IntoChildNode<
		GRAPH_ID,
		GRAPH_DEPTH,
		CHILD_INDEX,
		NODE_ID,
		PARENT_DEPTH,
		Marker,
	>;

	fn plugin(self) -> impl Plugin {
		AiPlugin::new(move || self.get_into_root_node())
	}
	fn bundle(self) -> impl Bundle {
		AiBundle::new(move || self.get_into_root_node())
	}
	fn bundle_inactive(self) -> impl Bundle {
		AiBundle::inactive(move || self.get_into_root_node())
	}
	fn node(self) -> impl AiNode { self.get_into_root_node().into_root_node() }
	fn node_state(self, world: &World, entity: Entity) -> Option<NodeState> {
		self.node().node_state(world, entity)
	}

	fn edge_state(self, world: &World, entity: Entity) -> Option<EdgeState> {
		self.node().edge_state(world, entity)
	}

	fn child(self, index: usize) -> Box<dyn NodeInspector> {
		self.node().child_owned(index)
	}
	fn graph_id(self) -> usize { NodeInspector::graph_id(&self.node()) }
	fn child_index(self) -> usize { NodeInspector::child_index(&self.node()) }
	fn graph_depth(self) -> usize { NodeInspector::graph_depth(&self.node()) }
	fn parent_depth(self) -> usize { NodeInspector::parent_depth(&self.node()) }
}

// implement for builders
impl<F, T> AiTree for F
where
	F: 'static + Send + Sync + Copy + FnOnce() -> T,
	T: AiTree,
{
	fn get_into_root_node(self) -> impl IntoRootNode {
		self().get_into_root_node()
	}

	fn get_into_child_node<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		const PARENT_DEPTH: usize,
		Marker,
	>(
		self,
	) -> impl IntoChildNode<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, PARENT_DEPTH,Marker>
	{
		self().get_into_child_node()
	}
}
