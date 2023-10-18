use super::*;
use crate::*;

pub struct TreeBundle;

impl TreeBundle {
	/// Recursively create a prop with the given value for each node in the tree.
	pub fn recursive<M, T: IntoProp + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		value: T,
	) -> Node::BundleRecursive<T> {
		Node::tree_bundle(value)
	}
	/// Create a prop with the given value for the root node only.
	pub fn root<M, T: IntoProp + Default + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		value: T,
	) -> Prop<T, Node> {
		Prop::new(value)
	}
}
