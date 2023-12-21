use gamai::action_list;
use gamai::prelude::*;

action_list!(MyNodes, [
	EmptyAction,
	SetRunResult,
	SetScore,
	SucceedInDuration,
	SequenceSelector,
	FallbackSelector,
	UtilitySelector
]);

pub fn main() {}
