use bevy_ecs::prelude::*;
use gamai::prelude::*;
use std::vec;


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
	ActionTree::new(vec![Box::new(TestAction::default())])
		.with_leaf(vec![Box::new(TestAction::default())])
		.with_child(
			ActionTree::new(vec![Box::new(TestAction::default())])
				.with_leaf(vec![Box::new(TestAction::default())]),
		)
		.into_action_graph()
}
