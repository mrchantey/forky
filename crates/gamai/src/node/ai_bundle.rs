use super::*;
use bevy_ecs::prelude::*;

// is clone required here?
#[derive(Debug, Bundle)]
pub struct AiBundle<Node: AiNode> {
	phantom: PhantomComponent<Node>,
	/// The edge state for this node.
	edge_state: DerefEdgeState<Node>,
	/// Nested AiBundle tuple, ie `(AiBundle<Child0>,(AiBundle<Child1>,..))`
	children: Node::ChildBundle,
}

impl<Node: AiNode> AiBundle<Node> {
	/// Initialize default with `NodeState::Running` for the root node.
	pub fn new<M>(_node: impl IntoNode<M, Out = Node>) -> impl Bundle {
		(
			DerefNodeState::<Node>::new(NodeState::Running),
			Self::default(),
		)
	}
	/// Initialize default, will not run until root node has `NodeState::Running`.
	pub fn inactive<M>(_node: impl IntoNode<M, Out = Node>) -> Self {
		Self::default()
	}
}

impl<Node: AiNode> Default for AiBundle<Node> {
	fn default() -> Self {
		Self {
			phantom: PhantomComponent::new(),
			edge_state: DerefEdgeState::new(EdgeState::default()),
			children: Node::ChildBundle::default(),
		}
	}
}
