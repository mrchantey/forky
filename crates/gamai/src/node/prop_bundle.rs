use super::*;

pub struct PropBundle;

impl PropBundle {
	/// Recursively create a prop with the given value for each node in the tree.
	pub fn recursive<M, T: IntoNodeComponent + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		value: T,
	) -> Node::TreeBundle<T> {
		Node::tree_bundle(value)
	}
	/// Create a prop with the given value for the root node.
	pub fn root<M, T: IntoNodeComponent + Default + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		val: T,
	) -> NodeComponent<T, Node> {
		NodeComponent::new(val)
	}
}
