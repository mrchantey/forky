use crate::*;
use bevy_ecs::prelude::*;

/// An action that removes the [Running] and [ActionResult] components from a node.
/// The first time it finds an [ActionResult] it will remove [Running] and optionally reset [ActionTimer].
/// It will allow the [ActionResult] to persist for a single frame before removing that as well.
#[action]
pub fn combined_post_update<N: AiNode>(
	mut commands: Commands,
	mut added_result: Query<
		(Entity, Option<&mut Prop<ActionTimer, N>>),
		(With<Prop<ActionResult, N>>, With<Prop<Running, N>>),
	>,
	// for second time around, remove the [ActionResult]
	mut removed_running: RemovedComponents<Prop<Running, N>>,
) {
	// First time around ensure this node doesnt run again
	// but dont yet remove the result
	// if the node has a timer, reset last_finish
	for (entity, timer) in added_result.iter_mut() {
		commands.entity(entity).remove::<Prop<Running, N>>();
		// println!("removing running");
		if let Some(mut timer) = timer {
			timer.last_finish.reset();
		}
	}


	for entity in removed_running.read() {
		if let Some(mut commands) = commands.get_entity(entity) {
			commands.remove::<Prop<ActionResult, N>>();
		}
	}
}

use crate::node::*;

// TODO deprecate
#[action]
pub fn remove_running<N: AiNode>(
	mut commands: Commands,
	added_result: Query<
		Entity,
		(With<Prop<ActionResult, N>>, With<Prop<Running, N>>),
	>,
	mut removed_running: RemovedComponents<Prop<Running, N>>,
	//TODO added_interrupt, recursive cleanup
) {
	for entity in added_result.iter() {
		commands.entity(entity).remove::<Prop<Running, N>>();
	}
	// Second time around ensure parent doesnt read state again
	for entity in removed_running.read() {
		commands.entity(entity).remove::<Prop<ActionResult, N>>();
	}
}
