use super::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub trait AiTree: 'static + Send + Sync + Copy {
	fn get_into_root_node(self) -> impl IntoRootNode;

	fn get_into_child_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
		self,
	) -> impl IntoChildNode<CHILD_INDEX, Parent>;
	
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

	fn get_into_child_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
		self,
	) -> impl IntoChildNode<CHILD_INDEX, Parent> {
		self().get_into_child_node()
	}
}
