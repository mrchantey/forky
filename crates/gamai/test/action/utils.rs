use gamai::prelude::*;

///
/// Root
/// 	Child0
/// 	Child1
/// 		Child0
///
// pub fn test_action_graph_boxed() -> BoxedActionGraph {
// 	BoxedActionTree::new(vec![Box::new(SetScore::default())])
// 		.with_leaf(vec![Box::new(SetScore::default())])
// 		.with_child(
// 			BoxedActionTree::new(vec![Box::new(SetScore::default())])
// 				.with_leaf(vec![Box::new(SetScore::default())]),
// 		)
// 		.into_action_graph()
// }

pub fn test_action_graph_typed() -> ActionGraph<BuiltinNode> {
	Tree::new(vec![BuiltinNode::SetScore(SetScore::default())])
		.with_leaf(vec![BuiltinNode::SetScore(SetScore::default())])
		.with_child(
			Tree::new(vec![BuiltinNode::SetScore(SetScore::default())])
				.with_leaf(vec![BuiltinNode::SetScore(SetScore::default())]),
		)
		.into_action_graph()
}
