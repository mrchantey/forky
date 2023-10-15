use crate::*;

pub struct PropBundle;

impl PropBundle {
	/// Recursively create a prop with the given value for each node in the tree.
	pub fn recursive<M, T: IntoProp + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		value: T,
	) -> Node::TreeBundle<T> {
		Node::tree_bundle(value)
	}
	/// Create a prop with the given value for the root node.
	pub fn root<M, T: IntoProp + Default + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		val: T,
	) -> Prop<T, Node> {
		Prop::new(val)
	}
}
