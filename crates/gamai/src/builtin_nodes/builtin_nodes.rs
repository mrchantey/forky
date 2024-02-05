use crate::prelude::*;
use gamai::action_list;

action_list!(BuiltinNode, [
	EmptyAction,
	SetRunResult,
	SetScore,
	SucceedInDuration,
	SequenceSelector,
	FallbackSelector,
	UtilitySelector
]);