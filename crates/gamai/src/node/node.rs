use crate::*;
use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;
// use std::marker::PhantomData;

// debug for edges to also be debug
/// An AiNode is a combination of a system and a set of child nodes, each with an edge.
pub trait AiNode:
	Bundle + std::fmt::Debug + Default + 'static + Send + Sync
{
	const NODE_ID: usize;
	const GRAPH_ID: usize;
	const GRAPH_DEPTH: usize;
	const CHILD_INDEX: usize;
	const PARENT_DEPTH: usize; //required until complex expressions https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#const-generics-with-complex-expressions
	type ChildrenQuery: WorldQuery;
	type NodeSystem: IntoNodeSystem;
	type EdgeSystem: IntoNodeSystem;
	// type Parent: AiNode;

	type Query<'w, 's> = Query<'w, 's, Self::ChildrenQuery>;
	fn edges(
		query: &Query<Self::ChildrenQuery>,
	) -> Vec<(Entity, Vec<EdgeState>)>;
	fn set_child_node_state(
		commands: &mut Commands,
		entity: Entity,
		index: usize,
	) -> Result<()>;

	fn build(schedule: &mut Schedule);
}

/* GENERIC ORDER:
NodeSystem,
EdgeSystem,
NODE_ID,
GRAPH_ID,
GRAPH_DEPTH,
CHILD_INDEX,
PARENT_DEPTH,
Child0,
Child1..
*/

#[derive(Debug, Default, Clone, Component)]
pub struct PhantomComponent<const NODE_ID: usize, T>(PhantomData<T>);

crate::define_node!(0);
crate::define_node!(1);
crate::define_node!(2);
crate::define_node!(3);
crate::define_node!(4);
crate::define_node!(5);
crate::define_node!(6);
crate::define_node!(7);
crate::define_node!(8);
crate::define_node!(9);
crate::define_node!(10);
crate::define_node!(11);
crate::define_node!(12);
crate::define_node!(13);
crate::define_node!(14);
crate::define_node!(15);
crate::define_node!(16);
