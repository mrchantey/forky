use crate::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;
use std::time::Duration;

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



#[action]
pub fn succeed_in_one_second<N: AiNode>(
	mut commands: Commands,
	mut query: Query<(Entity, &Prop<ActionTimer, N>), With<Prop<Running, N>>>,
) {
	for (entity, timer) in query.iter_mut() {
		if timer.last_start.elapsed() >= Duration::from_secs(1) {
			// println!("last start: {:?}", timer.last_start.elapsed());
			commands
				.entity(entity)
				.insert(Prop::<_, N>::new(ActionResult::Success));
		}
	}
}
