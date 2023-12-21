use crate::prelude::*;
use bevy_app::App;
use bevy_app::Update;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
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
	pub fn into_graph(self) -> ActionGraph { ActionGraph::from_tree(self) }
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

	pub fn from_graph<T: IntoAction>(graph: DiGraph<Vec<T>, ()>) -> Self {
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

	// pub fn spawn_with_commands(commands:&mut Commands){


	// }

	/// Can be called multiple times without duplication of systems
	pub fn add_systems(&self, app: &mut App) {
		self.add_systems_to_schedule(app, || {
			ActionSchedule::new(Update, PreTickSet, TickSet, PostTickSet)
		})
	}
	pub fn add_systems_to_schedule<
		Schedule: ScheduleLabel + Clone,
		PreTickSet: SystemSet + Clone,
		TickSet: SystemSet + Clone,
		PostTickSet: SystemSet + Clone,
	>(
		&self,
		app: &mut App,
		init_schedule: impl FnOnce() -> ActionSchedule<
			Schedule,
			PreTickSet,
			TickSet,
			PostTickSet,
		>,
	) {
		if false
			== app.world.contains_resource::<ActionSchedule<
				Schedule,
				PreTickSet,
				TickSet,
				PostTickSet,
			>>() {
			let tracker = init_schedule();


			app.configure_sets(
				tracker.schedule.clone(),
				tracker.pre_tick_set.clone(),
			);
			app.configure_sets(
				tracker.schedule.clone(),
				tracker.tick_set.clone().after(tracker.pre_tick_set.clone()),
			);
			app.configure_sets(
				tracker.schedule.clone(),
				tracker
					.post_tick_set
					.clone()
					.after(tracker.tick_set.clone()),
			);

			app.add_systems(
				tracker.schedule.clone(),
				apply_deferred
					.after(tracker.tick_set.clone())
					.before(tracker.post_tick_set.clone()),
			);
			app.add_systems(
				tracker.schedule.clone(),
				(sync_running, sync_interrupts)
					.in_set(tracker.post_tick_set.clone()),
			);

			app.world.insert_resource(tracker);
		}

		for actions in self.node_weights() {
			for action in actions {
				let mut action_schedule = app
					.world
					.get_resource_mut::<ActionSchedule<Schedule, PreTickSet, TickSet, PostTickSet>>(
					)
					.unwrap();

				if action_schedule.should_add_action(action.as_ref()) {
					let schedule = action_schedule.schedule.clone();
					let tick_set = action_schedule.tick_set.clone();
					let post_tick_set = action_schedule.post_tick_set.clone();
					app.add_systems(
						schedule.clone(),
						action.tick_system().in_set(tick_set),
					);
					app.add_systems(
						schedule,
						action.post_tick_system().in_set(post_tick_set),
					);
				}
			}
		}
	}
}
