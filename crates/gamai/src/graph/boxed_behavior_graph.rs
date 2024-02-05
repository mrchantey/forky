// use crate::prelude::*;
// use bevy_derive::Deref;
// use bevy_derive::DerefMut;
// use extend::ext;
// use petgraph::graph::DiGraph;
// use serde::Deserialize;
// use serde::Serialize;


// pub type BoxedAction = Box<dyn Action>;
// pub type BoxedBehaviorNode = BehaviorNode<BoxedAction>;
// pub type BoxedBehaviorTree = Tree<BoxedBehaviorNode>;

// impl IntoAction for BoxedAction {
// 	fn into_action(self) -> Box<dyn Action> { self }
// 	fn into_action_ref(&self) -> &dyn Action { self }
// 	fn into_action_mut(&mut self) -> &mut dyn Action { self }
// }

// impl Into<BoxedBehaviorTree> for Box<dyn Action> {
// 	fn into(self) -> BoxedBehaviorTree {
// 		BoxedBehaviorTree::new(vec![self.into()])
// 	}
// }

// impl BoxedBehaviorTree {
// 	pub fn from_action(value: impl Action) -> Self {
// 		let a = BehaviorNode::new(vec![Box::new(value)]);

// 		Self {
// 			value: BehaviorNode::new(vec![]),
// 			children: vec![],
// 		}
// 	}
	// pub fn into_action_graph(self) -> BoxedBehaviorGraph {
	// 	BoxedBehaviorGraph::from_tree(self)
	// }
// }

// #[derive(Default, Deref, DerefMut, Serialize, Deserialize)]
// pub struct BoxedBehaviorGraph(pub DiGraph<BoxedBehaviorNode, ()>);

// impl Clone for BoxedBehaviorGraph {
// 	fn clone(&self) -> Self {
// 		let graph = self.map(
// 			|_, action_list| {
// 				action_list
// 					.into_iter()
// 					.map(|action| action.duplicate())
// 					.collect::<Vec<_>>()
// 			},
// 			|_, _| (),
// 		);
// 		BoxedBehaviorGraph(graph)
// 	}
// }

// impl BoxedBehaviorGraph {
// 	pub fn new() -> Self { Self::default() }

// 	pub fn from_tree(root: impl Into<BoxedBehaviorTree>) -> Self {
// 		Self(DiGraph::from_tree(root.into()))
// 	}

// 	pub fn from_graph<T: IntoAction>(graph: &DiGraph<Vec<T>, ()>) -> Self {
// 		Self(graph.map(
// 			|_, action_list| {
// 				action_list
// 					.into_iter()
// 					.map(|action| action.clone().into_action())
// 					.collect::<Vec<_>>()
// 			},
// 			|_, _| (),
// 		))
// 	}

// / # Panics
// / Panics if the graph is empty.
// pub fn spawn(
// 	&self,
// 	world: &mut impl WorldOrCommands,
// 	target: Entity,
// ) -> EntityGraph {
// 	if self.node_count() == 0 {
// 		panic!("BehaviorGraph is empty");
// 	}

// 	// create entities & actions
// 	let entity_graph = self.map(
// 		|_, actions| {
// 			let entity = world.spawn((
// 				Name::from("Action Graph Node"),
// 				TargetEntity(target),
// 				RunTimer::default(),
// 			));

// 			for action in actions.iter() {
// 				world.apply_action(action.as_ref(), entity);
// 			}
// 			entity
// 		},
// 		|_, _| (),
// 	);

// 	// create edges
// 	for (index, entity) in Iterator::zip(
// 		entity_graph.node_indices(),
// 		entity_graph.node_weights(),
// 	) {
// 		let children = entity_graph
// 			.neighbors_directed(index, petgraph::Direction::Outgoing)
// 			.map(|index| entity_graph[index])
// 			.collect::<Vec<_>>();
// 		world.insert(*entity, Edges(children));
// 	}

// 	world.insert(*entity_graph.root().unwrap(), Running);

// 	EntityGraph(entity_graph)
// }
// }

// #[ext]
// pub impl BoxedBehaviorGraph {
// 	fn from_typed_graph<T: Clone + Into<Box<dyn Action>>>(
// 		graph: DiGraph<Vec<T>, ()>,
// 	) -> Self {
// 		let graph = graph.map(
// 			|_, action_list| {
// 				action_list
// 					.into_iter()
// 					.map(|action| action.clone().into())
// 					.collect::<Vec<_>>()
// 			},
// 			|_, _| (),
// 		);
// 		Self(graph)
// 	}
// 	fn into_typed_graph<T: From<Box<dyn Action>>>(self) -> DiGraph<Vec<T>, ()> {
// 		self.map(
// 			|_, action_list| {
// 				action_list
// 					.into_iter()
// 					.map(|action| action.duplicate().into())
// 					.collect::<Vec<_>>()
// 			},
// 			|_, _| (),
// 		)
// 	}
// }
