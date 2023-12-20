use bevy_ecs::prelude::*;
use gamai::prelude::*;


#[action(system=test_action)]
#[derive(Default, Clone, Component, Serialize, Deserialize)]
pub struct TestAction {
	#[shared]
	pub score: Score,
}
impl TestAction {
	pub fn new(score: Score) -> Self { Self { score } }
}

fn test_action() {}


///
/// Root
/// 	Child0
/// 	Child1
/// 		Child0
///
pub fn test_action_graph() -> ActionGraph {
	TestAction::default()
		.with_child(TestAction::default())
		.with_child(
			TestAction::default()
				.into_tree()
				.with_child(TestAction::default()),
		)
		.into_graph()
}
