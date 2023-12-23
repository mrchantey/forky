use gamai::prelude::*;

///
/// Root
/// 	Child0
/// 	Child1
/// 		Child0
///
pub fn test_action_graph() -> ActionGraph {
	ActionTree::new(vec![Box::new(SetScore::default())])
		.with_leaf(vec![Box::new(SetScore::default())])
		.with_child(
			ActionTree::new(vec![Box::new(SetScore::default())])
				.with_leaf(vec![Box::new(SetScore::default())]),
		)
		.into_action_graph()
}
