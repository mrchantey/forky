use gamai::prelude::*;
use petgraph::Graph;

///
/// Root
/// 	Child0
/// 	Child1
/// 		Child0
///
pub fn test_action_graph_boxed() -> BoxedActionGraph {
	BoxedActionTree::new(vec![Box::new(SetScore::default())])
		.with_leaf(vec![Box::new(SetScore::default())])
		.with_child(
			BoxedActionTree::new(vec![Box::new(SetScore::default())])
				.with_leaf(vec![Box::new(SetScore::default())]),
		)
		.into_action_graph()
}

pub fn test_action_graph_typed() -> ActionGraph<BuiltinNode> {
	let tree = Tree::new(vec![BuiltinNode::SetScore(SetScore::default())])
		.with_leaf(vec![BuiltinNode::SetScore(SetScore::default())])
		.with_child(
			Tree::new(vec![BuiltinNode::SetScore(SetScore::default())])
				.with_leaf(vec![BuiltinNode::SetScore(SetScore::default())]),
		);
	ActionGraph(tree.into_graph())
	// ActionGraph::from_other_graph()
	// tree.into_graph()
}
