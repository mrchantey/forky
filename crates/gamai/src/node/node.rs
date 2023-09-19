use crate::*;
use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
// use std::marker::PhantomData;

// debug for edges to also be debug
/// An AiNode is a combination of a system and a set of child nodes, each with an edge.
pub trait AiNode: std::fmt::Debug + Default + 'static + Send + Sync {
	const ID: usize;
	type ChildrenQuery: WorldQuery;
	type System: IntoNodeSystem;
	type Query<'w, 's> = Query<'w, 's, Self::ChildrenQuery>;
	fn edges(
		query: &Query<Self::ChildrenQuery>,
	) -> Vec<(Entity, Vec<EdgeState>)>;
	fn set_child_node_state(
		commands: &mut Commands,
		entity: Entity,
		index: usize,
	) -> Result<()>;
	// fn add_node_system<A: AiNode>(schedule: &mut Schedule, set: impl SystemSet);
}

// #[derive(Debug, Default, Copy, Clone)]
// pub struct AnonNode<Node: IntoNodeSystem, const ID: usize>(PhantomData<Node>);

// impl<Node: IntoNodeSystem, const ID: usize> AiNode for AnonNode<Node, ID> {
// 	type ChildrenQuery = ();
// 	const ID: usize = ID;
// 	fn edges(
// 		val: &Query<Self::ChildrenQuery>,
// 	) -> Vec<(Entity, Vec<EdgeState>)> {
// 		vec![]
// 	}

// 	fn set_child_node_state(
// 		commands: &mut Commands,
// 		entity: Entity,
// 		index: usize,
// 	) -> Result<()> {
// 		anyhow::bail!(Self::SET_CHILD_ERROR)
// 	}
// }

// impl<Node: IntoNodeSystem, const ID: usize> IntoNodeSystem
// 	for AnonNode<Node, ID>
// {
// 	fn add_node_system<N: AiNode>(
// 		schedule: &mut Schedule,
// 		set: impl SystemSet,
// 	) {
// 		Node::add_node_system::<N>(schedule, set)
// 	}
// }
