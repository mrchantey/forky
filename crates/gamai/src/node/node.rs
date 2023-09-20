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
	fn set_pre_update() -> impl SystemSet;
	fn set_update() -> impl SystemSet;
	fn set_post_update() -> impl SystemSet;
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

// crate::node!(0);
// crate::node!(1);
// crate::node!(2);
// crate::node!(3);
// crate::node!(4);
// crate::node!(5);
// crate::node!(6);
// crate::node!(7);
// crate::node!(8);
// crate::node!(9);
// crate::node!(10);
// crate::node!(11);
// crate::node!(12);
// crate::node!(13);
// crate::node!(14);
// crate::node!(15);
// crate::node!(16);



// #[derive(Debug, Default)]
// pub struct RootParentNode<const GRAPH_ID: usize>;

// impl<const GRAPH_ID: usize> AiNode for RootParentNode<GRAPH_ID> {
// 	const NODE_ID: usize = 0;
// 	const GRAPH_ID: usize = GRAPH_ID;
// 	const GRAPH_DEPTH: usize = 0;
// 	const CHILD_INDEX: usize = 0;
// 	type ChildrenQuery = ();
// 	type NodeSystem = noop_node;
// 	type EdgeSystem = noop_node;
// 	// type Parent = RootParentNode<0>;
// 	type Query<'w, 's> = Query<'w, 's, Self::ChildrenQuery>;
// 	fn edges(
// 		query: &Query<Self::ChildrenQuery>,
// 	) -> Vec<(Entity, Vec<EdgeState>)> {
// 		panic!("not implemented for RootParentNode")
// 	}
// 	fn set_child_node_state(
// 		commands: &mut Commands,
// 		entity: Entity,
// 		index: usize,
// 	) -> Result<()> {
// 		panic!("not implemented for RootParentNode")
// 	}

// 	fn build(schedule: &mut Schedule) {
// 		panic!("not implemented for RootParentNode")
// 	}
// 	fn node_pre_update() -> impl SystemSet { NodePreUpdate::<GRAPH_ID, 0> }
// 	fn node_update() -> impl SystemSet { NodeUpdate::<GRAPH_ID, 0> }
// 	fn node_post_update() -> impl SystemSet { NodePostUpdate::<GRAPH_ID, 0> }
// }


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
