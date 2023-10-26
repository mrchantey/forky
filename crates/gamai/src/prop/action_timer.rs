use bevy_time::Stopwatch;
use std::time::Duration;

/// Prop for tracking the last time an action was started and finished.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ActionTimer {
	/// Resets whenever a [Running] component is added.
	pub last_start: Stopwatch,
	/// Resets whenever a [Running] component is removed.
	pub last_finish: Stopwatch,
}


impl ActionTimer {
	/// Calculated as [ActionTimer::last_finish] - [ActionTimer::last_start]
	pub fn last_duration(&self) -> Duration {
		self.last_finish.elapsed() - self.last_start.elapsed()
	}
}
