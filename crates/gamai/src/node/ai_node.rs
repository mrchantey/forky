use crate::*;
use bevy_app::Plugin;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;


/// An AiNode is a node and edge system, and a set of child nodes.
// pub trait AiNode {
pub trait AiNode: 'static + Send + Sync + TreePath {
	/// Tuple Query used to access child states: `(Entity,(Child1,(Child2)))`
	type ChildQuery<T: IntoNodeComponent>: WorldQuery;
	type ChildQueryOpt<T: IntoNodeComponent>: WorldQuery;
	type ChildQueryMut<T: IntoNodeComponent>: WorldQuery;
	type ChildQueryOptMut<T: IntoNodeComponent>: WorldQuery;

	type TreeBundle<T: IntoNodeComponent>: Bundle;
	// type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;
	fn entity<'a, T: IntoNodeComponent>(
		item: &<Self::ChildQuery<T> as WorldQuery>::Item<'a>,
	) -> Entity;
	fn children<'a, T: IntoNodeComponent>(
		item: <Self::ChildQuery<T> as WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a, T>>;
	fn children_opt<'a, T: IntoNodeComponent>(
		item: <Self::ChildQueryOpt<T> as WorldQuery>::Item<'a>,
	) -> Vec<ChildStateOpt<'a, T>>;
	fn children_mut<'a, T: IntoNodeComponent>(
		item: <Self::ChildQueryMut<T> as WorldQuery>::Item<'a>,
	) -> Vec<ChildStateMut<'a, T>>;
	fn children_opt_mut<'a, T: IntoNodeComponent>(
		item: <Self::ChildQueryOptMut<T> as WorldQuery>::Item<'a>,
	) -> Vec<ChildStateOptMut<'a, T>>;

	fn add_systems(self, schedule: &mut Schedule);

	fn tree_bundle<T: IntoNodeComponent + Clone>(
		value: T,
	) -> Self::TreeBundle<T>;

	fn get_child(&self, index: usize) -> &dyn NodeInspector;
	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector>;
	fn get_children(&self) -> Vec<&dyn NodeInspector>;
	fn get_children_owned(self) -> Vec<Box<dyn NodeInspector>>;


	fn get_recursive<T: IntoNodeComponent>(
		self,
		world: &World,
		entity: Entity,
	) -> NodeComponentRecursive<T> {
		self.get_recursive_inner::<T>(world, entity, 0)
	}
	fn get_recursive_inner<T: IntoNodeComponent>(
		self,
		world: &World,
		entity: Entity,
		depth: usize,
	) -> NodeComponentRecursive<T>;

	/// Copies self, with a different path.
	fn into_child<Path: TreePath>(self) -> impl AiNode;
	/// Fixes paths of all children to be relative to self.
	fn into_root(self) -> impl AiNode { self.into_child::<Self>() }

	fn plugin(self) -> impl Plugin { AiPlugin::new(self) }
}

#[derive(Debug, Default, Clone, Component)]
pub struct PhantomComponent<T>(pub PhantomData<T>);

impl<T> PhantomComponent<T> {
	pub fn new() -> Self { Self(PhantomData) }
}

/// Base type for nodes, must be Send/Sync because used in components for distinguishing nodes
pub trait NodeInspector: 'static + Send + Sync {
	// fn node_inspector_into_node(self) -> impl AiNode;
	fn child(&self, index: usize) -> &dyn NodeInspector;
	fn child_owned(self, index: usize) -> Box<dyn NodeInspector>;
	fn children(&self) -> Vec<&dyn NodeInspector>;
	fn children_owned(self) -> Vec<Box<dyn NodeInspector>>;
	fn graph_id(&self) -> usize;
	fn child_index(&self) -> usize;
	fn graph_depth(&self) -> usize;
}

impl<T> NodeInspector for T
where
	T: AiNode + Sized,
{
	// fn node_inspector_into_node(self) -> impl AiNode { self }

	fn graph_id(&self) -> usize { Self::GRAPH_ID }
	fn child_index(&self) -> usize { Self::CHILD_INDEX }
	fn graph_depth(&self) -> usize { Self::DEPTH }

	fn child(&self, index: usize) -> &dyn NodeInspector {
		self.get_child(index)
	}
	fn child_owned(self, index: usize) -> Box<dyn NodeInspector> {
		self.get_child_owned(index)
	}

	fn children(&self) -> Vec<&dyn NodeInspector> { self.get_children() }

	fn children_owned(self) -> Vec<Box<dyn NodeInspector>> {
		self.get_children_owned()
	}
}
