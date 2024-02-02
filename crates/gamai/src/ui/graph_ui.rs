use crate::prelude::*;
use extend::ext;
use petgraph::graph::DiGraph;



#[ext]
pub impl<N: IntoFieldUi, E> DiGraph<N, E> {
	fn into_field_ui_roots(&self) -> Vec<FieldUiRoot<N>> {
		self.raw_nodes()
			//TODO how to into_iter?
			.iter()
			.map(|n| FieldUiRoot::new(n.weight.clone()))
			.collect()
	}
}
