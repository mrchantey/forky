use crate::prelude::*;
use extend::ext;
use petgraph::graph::DiGraph;



pub type FieldGraph<T> = DiGraph<Vec<FieldUiRoot<T>>, ()>;

#[ext]
pub impl<N: IntoFieldUi, E> DiGraph<Vec<N>, E> {
	fn into_field_graph(&self) -> FieldGraph<N> {
		self.map(
			|_i, node_actions| {
				node_actions
					.iter()
					.map(|n| FieldUiRoot::new(n.clone()))
					.collect()
			},
			|_i, _| (),
		)
	}
}
