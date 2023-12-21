use gamai::node_list;
use gamai::prelude::*;

node_list!(MyNodes, [
	EmptyAction,
	SetRunResult,
	SetScore,
	SucceedInDuration,
	SequenceSelector,
	FallbackSelector,
	UtilitySelector
]);

pub fn main() {}
