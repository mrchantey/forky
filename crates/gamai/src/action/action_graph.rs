use crate::prelude::*;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use petgraph::graph::DiGraph;
use serde::Deserialize;
use serde::Serialize;

pub type ActionList = Vec<Box<dyn Action>>;
pub type ActionTree = Tree<ActionList>;

impl Into<ActionTree> for Box<dyn Action> {
	fn into(self) -> ActionTree { ActionTree::new(vec![self]) }
}

impl ActionTree {
	pub fn from_action(value: impl Action) -> Self {
		Self {
			value: vec![Box::new(value)],
			children: vec![],
		}
	}
	pub fn into_action_graph(self) -> ActionGraph {
		ActionGraph::from_tree(self)
	}
}

#[derive(Default, Deref, DerefMut, Serialize, Deserialize)]
pub struct ActionGraph(pub DiGraph<ActionList, ()>);

impl Clone for ActionGraph {
	fn clone(&self) -> Self {
		let graph = self.map(
			|_, action_list| {
				action_list
					.into_iter()
					.map(|action| action.duplicate())
					.collect::<Vec<_>>()
			},
			|_, _| (),
		);
		ActionGraph(graph)
	}
}

impl ActionGraph {
	pub fn new() -> Self { Self::default() }

	pub fn from_tree(root: impl Into<ActionTree>) -> Self {
		Self(DiGraph::from_tree(root.into()))
	}

	pub fn from_graph<T: IntoAction>(graph: &DiGraph<Vec<T>, ()>) -> Self {
		Self(graph.map(
			|_, action_list| {
				action_list
					.into_iter()
					.map(|action| action.clone().into_action())
					.collect::<Vec<_>>()
			},
			|_, _| (),
		))
	}


	pub fn spawn(
		&self,
		world: &mut impl WorldOrCommands,
		target: Entity,
	) -> EntityGraph {
		// create entities & actions
		let entity_graph = self.map(
			|_, actions| {
				let entity =
					world.spawn((TargetEntity(target), RunTimer::default()));

				for action in actions.iter() {
					world.apply_action(action.as_ref(), entity);
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
