use crate::*;
use bevy_ecs::prelude::*;

/// An action that combines [common_actions::remove_running] and [common_actions::update_action_timer]
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
	for (entity, timer) in added_result.iter_mut() {
		commands.entity(entity).remove::<Prop<Running, N>>();
		if let Some(mut timer) = timer {
			timer.last_finish.reset();
		}
	}

	for entity in removed_running.iter() {
		commands.entity(entity).remove::<Prop<ActionResult, N>>();
	}
}
