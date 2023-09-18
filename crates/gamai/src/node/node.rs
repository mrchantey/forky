use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;

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
	fn add_node_system<A: AiNode>(schedule: &mut Schedule, set: impl SystemSet);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct LeafNode<System: IntoNodeSystem, const ID: usize>(
	PhantomData<System>,
);

impl<System: IntoNodeSystem, const ID: usize> AiNode for LeafNode<System, ID> {
	type ChildrenQuery = ();
	fn edges(_: &Query<Self::ChildrenQuery>) -> Vec<(Entity, Vec<EdgeState>)> {
		Vec::new()
	}

	fn set_child_node_state(_: &mut Commands, _: Entity, _: usize) {
		panic!("{}", Self::SET_CHILD_ERROR)
	}

	fn add_node_system<A: AiNode>(
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		// todo!()
		System::add_node_system::<A>(schedule, set)
	}
}

pub trait IntoNodeSystem:
	'static + std::fmt::Debug + Default + Clone + Send + Sync
{
	fn add_node_system<A: AiNode>(schedule: &mut Schedule, set: impl SystemSet);
}
