use crate::*;
use bevy_ecs::prelude::*;

/// Updates the start time of nodes that have just started running
#[action]
pub fn combined_pre_update<N: AiNode>(
	mut added_running: Query<
		&mut Prop<ActionTimer, N>,
		Added<Prop<Running, N>>,
	>,
) {
	for mut timer in added_running.iter_mut() {
		timer.last_start.reset();
	}
}
