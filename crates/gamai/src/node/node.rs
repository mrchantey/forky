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

crate::node!(0);
crate::node!(1);
crate::node!(2);
crate::node!(3);
crate::node!(4);
crate::node!(5);
crate::node!(6);
crate::node!(7);
crate::node!(8);
crate::node!(9);
crate::node!(10);
crate::node!(11);
crate::node!(12);
crate::node!(13);
crate::node!(14);
crate::node!(15);
crate::node!(16);

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
