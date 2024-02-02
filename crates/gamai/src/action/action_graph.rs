use crate::prelude::*;
use bevy_core::Name;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use petgraph::graph::DiGraph;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

// pub type ActionList<T> = Vec<T>;
pub type ActionTree<T> = Tree<Vec<T>>;

pub trait ActionSuper: Action + PartialEq {}
impl<T: Action + PartialEq> ActionSuper for T {}

#[derive(Default, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct ActionGraph<T: ActionSuper>(pub DiGraph<Vec<T>, ()>);

impl<T: ActionSuper> PartialEq for ActionGraph<T> {
	fn eq(&self, other: &Self) -> bool { self.0.is_identical(other) }
}

impl<T: Debug + ActionSuper> Debug for ActionGraph<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
		// f.debug_tuple("ActionGraph").field(&self.0).finish()
	}
}


#[extend::ext]
pub impl<T: ActionSuper> ActionTree<T> {
	fn into_action_graph(self) -> ActionGraph<T> {
		ActionGraph(self.into_graph())
	}
}

impl<T: ActionSuper> ActionGraph<T> {
	pub fn new() -> Self { Self(DiGraph::new()) }
	pub fn from_tree(tree: ActionTree<T>) -> Self {
		Self(DiGraph::from_tree(tree))
	}


	pub fn spawn(
		&self,
		world: &mut impl WorldOrCommands,
		target: Entity,
	) -> EntityGraph {
		if self.node_count() == 0 {
			panic!("ActionGraph is empty");
		}

		// create entities & actions
		let entity_graph = self.map(
			|_, actions| {
				let entity = world.spawn((
					Name::from("Action Graph Node"),
					TargetEntity(target),
					RunTimer::default(),
				));

				for action in actions.iter() {
					world.apply_action_typed(action, entity);
				}
				entity
			},
			|_, _| (),
		);

		// create edges
		for (index, entity) in Iterator::zip(
			entity_graph.node_indices(),
			entity_graph.node_weights(),
		) {
			let children = entity_graph
				.neighbors_directed(index, petgraph::Direction::Outgoing)
				.map(|index| entity_graph[index])
				.collect::<Vec<_>>();
			world.insert(*entity, Edges(children));
		}

		world.insert(*entity_graph.root().unwrap(), Running);

		EntityGraph(entity_graph)
	}
}
