use crate::prelude::*;
use extend::ext;
use gamai::action_list;
use petgraph::graph::DiGraph;


action_list!(BuiltinNode, [
	EmptyAction,
	SetRunResult,
	SetScore,
	SucceedInDuration,
	SequenceSelector,
	FallbackSelector,
	UtilitySelector
]);



#[ext]
pub impl ActionGraph {
	fn from_other_graph<T: Clone + Into<Box<dyn Action>>>(
		graph: DiGraph<Vec<T>, ()>,
	) -> Self {
		let graph = graph.map(
			|_, action_list| {
				action_list
					.into_iter()
					.map(|action| action.clone().into())
					.collect::<Vec<_>>()
			},
			|_, _| (),
		);
		Self(graph)
	}
	fn into_other_graph<T: From<Box<dyn Action>>>(self) -> DiGraph<Vec<T>, ()> {
		self.map(
			|_, action_list| {
				action_list
					.into_iter()
					.map(|action| action.duplicate().into())
					.collect::<Vec<_>>()
			},
			|_, _| (),
		)
	}
}
