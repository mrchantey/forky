use crate::*;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

// debug for choices to also be debug
pub trait Agent: std::fmt::Debug + Default + 'static + Send + Sync {
	type Items: WorldQuery;
	type Query<'w, 's> = Query<'w, 's, Self::Items>;
	fn edges(query: &Query<Self::Items>) -> Vec<(Entity, Vec<EdgeState>)>;
	fn set_action(commands: &mut Commands, entity: Entity, index: usize);
}

pub trait AddAgentSystem: 'static + Clone + Send + Sync {
	fn add_agent_system<A: Agent>(&self, app: &mut App, set: impl SystemSet);
}
