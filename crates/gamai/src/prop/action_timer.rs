use crate::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;
use std::time::Instant;

/// Prop for tracking the last time an action was started and finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionTimer {
	/// The time the [Running] component was last added.
	pub last_start: Instant,
	/// The time the [Running] component was last removed.
	pub last_finish: Instant,
}

#[action]
pub fn update_action_timer<N: AiNode>(
	time: Res<Time>,
	mut timers: Query<&mut Prop<ActionTimer, N>>,
	added: Query<Entity, Added<Prop<Running, N>>>,
	mut removed: RemovedComponents<Prop<Running, N>>,
) {
	for added in added.iter() {
		if let Ok(mut timer) = timers.get_mut(added) {
			timer.last_start = time.startup() + time.elapsed();
		}
	}
	for removed in removed.iter() {
		if let Ok(mut timer) = timers.get_mut(removed) {
			timer.last_finish = time.startup() + time.elapsed();
		}
	}
}
