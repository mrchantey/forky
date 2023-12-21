use gamai::action_list;
use gamai::prelude::*;
use sweet::*;

action_list!(MyNodes, [
	EmptyAction,
	SetRunResult,
	SetScore,
	SucceedInDuration,
	SequenceSelector,
	FallbackSelector,
	UtilitySelector
]);


#[sweet_test]
pub fn works() -> Result<()> {
	// expect(true).to_be_false()?;

	// for item in MyNodes::iter() {
	// 	println!("{:?}", item);
	// }

	Ok(())
}
