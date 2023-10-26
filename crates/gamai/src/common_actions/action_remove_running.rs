use crate::node::*;
use crate::*;
use bevy_ecs::prelude::*;



/// An action that removes the [Running] and [ActionResult] components from a node.
/// The first time it finds an [ActionResult] it will remove [Running]
/// It will allow the [ActionResult] to persist for a single frame before removing that as well.
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
	// First time around ensure this node doesnt run again
	// but dont yet remove the result
	for entity in added_result.iter() {
		// println!("removing running");
		commands.entity(entity).remove::<Prop<Running, N>>();
	}
	// Second time around ensure parent doesnt read state again
	for entity in removed_running.iter() {
		// println!("removing result");
		commands.entity(entity).remove::<Prop<ActionResult, N>>();
	}
}
