use crate::*;
use bevy_app::Plugin;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;
// use std::marker::PhantomData;


// debug for edges to also be debug
/// An AiNode is a node and edge system, and a set of child nodes.
pub trait AiNode:
	Bundle + std::fmt::Debug + Default + 'static + Send + Sync
{
	const NODE_ID: usize;
	const GRAPH_ID: usize;
	const GRAPH_DEPTH: usize;
	const CHILD_INDEX: usize;
	const PARENT_DEPTH: usize; //required until complex expressions https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#const-generics-with-complex-expressions
	/// Tuple Query used to access child states: `(Entity,(Child1,(Child2)))`
	type ChildQuery: WorldQuery;
	/// System to run in this node's update set
	type NodeSystem: IntoNodeSystem;
	/// System to run in this node's preupdate set
	type EdgeSystem: IntoNodeSystem;

	type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;
	fn entity<'a>(item: &<Self::ChildQuery as WorldQuery>::Item<'a>) -> Entity;
	fn children<'a>(
		item: <Self::ChildQuery as WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a>>;

	fn add_systems(schedule: &mut Schedule);
	fn plugin() -> impl Plugin;
	fn bundle() -> impl Bundle;
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



// #[derive(Component)]
// struct Bar;

// fn foobar(mut query: Query<&mut Bar>) {
// 	for mut item in query.iter_mut() {
// 		*item = Bar;
// 	}
// }
