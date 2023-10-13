use crate::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;


pub struct IntoNodeMarkerNode;
pub struct IntoNodeMarkerFunc;

pub trait IntoNode<Marker>: Sized {
	type Out: AiNode;
	fn into_node(self) -> Self::Out;

	/// wrapper for `NodeInspector::node_state`
	fn node_state(self, world: &World, entity: Entity) -> Option<NodeState> {
		NodeInspector::node_state(&self.into_node(), world, entity)
	}

	/// wrapper for `NodeInspector::edge_state`
	fn edge_state(self, world: &World, entity: Entity) -> Option<EdgeState> {
		NodeInspector::edge_state(&self.into_node(), world, entity)
	}

	/// wrapper for `NodeInspector::child_owned`
	fn child(self, index: usize) -> Box<dyn NodeInspector> {
		NodeInspector::child_owned(self.into_node(), index)
	}

	/// wrapper for `NodeInspector::graph_id`
	fn graph_id(self) -> usize { NodeInspector::graph_id(&self.into_node()) }

	/// wrapper for `NodeInspector::child_index`
	fn child_index(self) -> usize {
		NodeInspector::child_index(&self.into_node())
	}

	/// wrapper for `NodeInspector::graph_depth`
	fn graph_depth(self) -> usize {
		NodeInspector::graph_depth(&self.into_node())
	}
}


impl<T: AiNode> IntoNode<IntoNodeMarkerNode> for T {
	type Out = T;
	fn into_node(self) -> Self::Out { self }
}



impl<Node: AiNode, Func: FnOnce() -> Node> IntoNode<IntoNodeMarkerFunc>
	for Func
{
	type Out = Node;
	fn into_node(self) -> Self::Out { self() }
}
