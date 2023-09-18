use crate::*;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

// debug for choices to also be debug
pub trait AiNode: std::fmt::Debug + Default + 'static + Send + Sync {
	type ChildrenQuery: WorldQuery;
	type Query<'w, 's> = Query<'w, 's, Self::ChildrenQuery>;
	fn edges(
		query: &Query<Self::ChildrenQuery>,
	) -> Vec<(Entity, Vec<EdgeState>)>;
	fn set_action(commands: &mut Commands, entity: Entity, index: usize);
}

pub trait AddAiNodeSystem: 'static + Clone + Send + Sync {
	fn add_node_system<A: AiNode>(&self, app: &mut App, set: impl SystemSet);
}
