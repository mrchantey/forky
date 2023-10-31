use super::*;

/// marker for any AiNode, ie `tree!{<my_action/>}`
pub struct IntoNodeMarkerNode;
/// marker for any func that returns an AiNode, ie `|| tree!{<my_action/>}`
pub struct IntoNodeMarkerFunc;

/// Trait for nodes and functions that return nodes.
pub trait IntoNode<Marker>: Sized {
	type Out: AiNode;
	fn into_node(self) -> Self::Out;

	/// wrapper for `NodeInspector::child_owned`
	fn child(self, index: usize) -> Box<dyn NodeInspector> {
		NodeInspector::child_owned(self.into_node(), index)
	}

	/// wrapper for `NodeInspector::graph_id`
	fn graph_id(self) -> usize { NodeInspector::graph_id(&self.into_node()) }

	/// wrapper for `NodeInspector::child_index`
	fn child_index(self) -> usize {
		NodeInspector::child_index(&self.into_node())
	}

	/// wrapper for `NodeInspector::graph_depth`
	fn graph_depth(self) -> usize {
		NodeInspector::graph_depth(&self.into_node())
	}
}


impl<T: AiNode> IntoNode<IntoNodeMarkerNode> for T {
	type Out = T;
	fn into_node(self) -> Self::Out { self }
}

impl<Node: AiNode, Func: FnOnce() -> Node> IntoNode<IntoNodeMarkerFunc>
	for Func
{
	type Out = Node;
	fn into_node(self) -> Self::Out { self() }
}
