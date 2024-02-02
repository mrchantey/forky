use super::*;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn boxed() -> Result<()> {
	let actions1 = test_action_graph_typed();
	let str1 = serde_json::to_string_pretty(&actions1)?;
	let actions2 = serde_json::from_str::<BoxedActionGraph>(&str1)?;
	let str2 = serde_json::to_string_pretty(&actions2)?;

	expect(&str1).to_be(&str2)?;

	Ok(())
}
#[sweet_test]
pub fn typed() -> Result<()> {
	let actions1 = test_action_graph_typed();
	let str1 = serde_json::to_string_pretty(&actions1)?;
	let actions2 = serde_json::from_str::<ActionGraph<BuiltinNode>>(&str1)?;
	let str2 = serde_json::to_string_pretty(&actions2)?;

	expect(&str1).to_be(&str2)?;

	Ok(())
}
