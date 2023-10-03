use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;

/// An AiNode is a node and edge system, and a set of child nodes.
pub trait AiNode: 'static + Send + Sync {
	const NODE_ID: usize;
	const GRAPH_ID: usize;
	const GRAPH_DEPTH: usize;
	const CHILD_INDEX: usize;
	const PARENT_DEPTH: usize; //required until complex expressions https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#const-generics-with-complex-expressions
	/// Tuple Query used to access child states: `(Entity,(Child1,(Child2)))`
	type ChildQuery: WorldQuery;
	type ChildBundle: 'static + Send + Sync + Default + Bundle;

	type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;
	fn entity<'a>(item: &<Self::ChildQuery as WorldQuery>::Item<'a>) -> Entity;
	fn children<'a>(
		item: <Self::ChildQuery as WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a>>;

	fn add_systems(self, schedule: &mut Schedule);
}

/* GENERIC ORDER:
NODE_ID,
GRAPH_ID,
GRAPH_DEPTH,
CHILD_INDEX,
PARENT_DEPTH,
Child0,
Child1..
*/

#[derive(Debug, Default, Clone, Component)]
pub struct PhantomComponent<T>(pub PhantomData<T>);

impl<T> PhantomComponent<T> {
	pub fn new() -> Self { Self(PhantomData) }
}
