use crate::*;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

// debug for edges to also be debug


pub trait AiNode: std::fmt::Debug + Default + 'static + Send + Sync {
	const SET_CHILD_ERROR: &'static str = "gamai: child index out of range";
	type ChildrenQuery: WorldQuery;
	type Query<'w, 's> = Query<'w, 's, Self::ChildrenQuery>;
	fn edges(
		query: &Query<Self::ChildrenQuery>,
	) -> Vec<(Entity, Vec<EdgeState>)>;
	fn set_child_node_state(
		commands: &mut Commands,
		entity: Entity,
		index: usize,
	);
	// fn add_node_system<A: AiNode>(&self, app: &mut App, set: impl SystemSet);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct LeafNode<const ID: usize>;

impl<const ID: usize> AiNode for LeafNode<ID> {
	type ChildrenQuery = ();
	fn edges(
		_: &Query<Self::ChildrenQuery>,
	) -> Vec<(Entity, Vec<EdgeState>)> {
		Vec::new()
	}

	fn set_child_node_state(
		_: &mut Commands,
		_: Entity,
		_: usize,
	) {
		panic!("{}",Self::SET_CHILD_ERROR)
	}

	// fn add_node_system<A: AiNode>(&self, app: &mut App, set: impl SystemSet) {
	// 	todo!()
	// }
}

pub trait AddAiNodeSystem: 'static + Clone + Send + Sync {
	fn add_node_system<A: AiNode>(&self, app: &mut App, set: impl SystemSet);
}
