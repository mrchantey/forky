use super::*;

pub trait IntoTree {
	fn build_root() -> impl AiNode;
	fn build_child<
		const CHILD_INDEX: usize,
		const GRAPH_ID: usize,
		const PARENT_DEPTH: usize,
		const GRANDPARENT_DEPTH: usize,
		Parent,
	>() -> impl AiNode
	where
		Parent: IntoNodeId<
			GRAPH_ID = { GRAPH_ID },
			GRAPH_DEPTH = { PARENT_DEPTH },
			PARENT_DEPTH = { GRANDPARENT_DEPTH },
		>;
}
