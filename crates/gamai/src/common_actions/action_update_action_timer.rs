use crate::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;


/// An action that updates the action timer for a given node.
/// The timer will update even if the node is not running.
#[action]
pub fn update_action_timer<N: AiNode>(
	time: Option<Res<Time>>,
	mut timers: Query<&mut Prop<ActionTimer, N>>,
) {
	if let Some(time) = time {
		for mut timer in timers.iter_mut() {
			timer.last_start.tick(time.delta());
			timer.last_finish.tick(time.delta());
		}
	} else if timers.iter().next().is_some() {
		eprintln!("Gamai - `ActionTimer` found but `Time` resource is missing");
	}
}