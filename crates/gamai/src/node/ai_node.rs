use super::*;
use crate::prop::*;
use bevy_app::Plugin;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;


/// An AiNode is a `TreePath` with actions and typed children.
// pub trait AiNode {
pub trait AiNode: 'static + Send + Sync + TreePath + IntoBundle {
	/// Tuple Query used to access child states: `(Entity,(Child1,(Child2)))`
	type ChildQuery<T: IntoProp>: WorldQuery;
	type ChildQueryOpt<T: IntoProp>: WorldQuery;
	type ChildQueryMut<T: IntoProp>: WorldQuery;
	type ChildQueryOptMut<T: IntoProp>: WorldQuery;

	type BundleRecursive<T: IntoProp>: Bundle;
	// type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;
	fn entity<'a, T: IntoProp>(
		item: &<Self::ChildQuery<T> as WorldQuery>::Item<'a>,
	) -> Entity;
	fn children<'a, T: IntoProp>(
		item: <Self::ChildQuery<T> as WorldQuery>::Item<'a>,
	) -> Vec<Box<dyn IntoChildProp<'a, T> + 'a>>;
	fn children_opt<'a, T: IntoProp>(
		item: <Self::ChildQueryOpt<T> as WorldQuery>::Item<'a>,
	) -> Vec<Box<dyn IntoChildPropOpt<'a, T> + 'a>>;
	fn children_mut<'a, T: IntoProp>(
		item: <Self::ChildQueryMut<T> as WorldQuery>::Item<'a>,
	) -> Vec<Box<dyn IntoChildPropMut<'a, T> + 'a>>;
	fn children_opt_mut<'a, T: IntoProp>(
		item: <Self::ChildQueryOptMut<T> as WorldQuery>::Item<'a>,
	) -> Vec<Box<dyn IntoChildPropOptMut<'a, T> + 'a>>;
	// fn parse_query<'a, T: IntoProp, Q: IntoChildQuery<Self, T>>(
	// 	item: Q,
	// ) -> Q::Out {
	// 	item.out()
	// }

	fn add_systems(self, schedule: &mut Schedule);

	fn tree_bundle<T: IntoProp + Clone>(value: T) -> Self::BundleRecursive<T>;

	fn get_child(&self, index: usize) -> &dyn NodeInspector;
	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector>;
	fn get_children(&self) -> Vec<&dyn NodeInspector>;
	fn get_children_owned(self) -> Vec<Box<dyn NodeInspector>>;


	fn get_recursive<T: IntoProp>(
		self,
		world: &World,
		entity: Entity,
	) -> PropTree<T> {
		self.get_recursive_inner::<T>(world, entity, 0)
	}
	fn get_recursive_inner<T: IntoProp>(
		self,
		world: &World,
		entity: Entity,
		depth: usize,
	) -> PropTree<T>;

	/// Copies self, with a different path.
	fn into_child<Path: TreePath>(self) -> impl AiNode;
	/// Fixes paths of all children to be relative to self.
	fn into_root(self) -> impl AiNode { self.into_child::<Self>() }

	fn plugin(self) -> impl Plugin { TreePlugin::new(self) }
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
