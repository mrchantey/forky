use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_utils::HashSet;

#[derive(Resource)]
pub struct ActionSchedule<
	Schedule: ScheduleLabel + Clone,
	PreTickSet: SystemSet + Clone,
	TickSet: SystemSet + Clone,
	PostTickSet: SystemSet + Clone,
> {
	pub added: HashSet<usize>,
	pub schedule: Schedule,
	pub pre_tick_set: PreTickSet,
	pub tick_set: TickSet,
	pub post_tick_set: PostTickSet,
}

impl<
		Schedule: ScheduleLabel + Clone,
		PreTickSet: SystemSet + Clone,
		TickSet: SystemSet + Clone,
		PostTickSet: SystemSet + Clone,
	> ActionSchedule<Schedule, PreTickSet, TickSet, PostTickSet>
{
	pub fn new(
		schedule: Schedule,
		pre_tick_set: PreTickSet,
		tick_set: TickSet,
		post_tick_set: PostTickSet,
	) -> Self {
		Self {
			added: HashSet::default(),
			schedule,
			pre_tick_set,
			tick_set,
			post_tick_set,
		}
	}

	pub fn should_add_action(&mut self, action: &dyn Action) -> bool {
		if self.added.contains(&action.meta().id) {
			return false;
		}
		self.added.insert(action.meta().id);
		true
	}
}
