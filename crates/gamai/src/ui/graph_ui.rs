use crate::prelude::*;
use extend::ext;
use petgraph::graph::DiGraph;



#[ext]
pub impl<N: IntoFieldUi, E> DiGraph<Vec<N>, E> {
	fn into_field_ui_roots(&self) -> Vec<Vec<FieldUiRoot<N>>> {
		self.raw_nodes()
			//TODO how to into_iter?
			.iter()
			.map(|n| {
				n.weight
					.iter()
					.map(|n| FieldUiRoot::new(n.clone()))
					.collect()
			})
			.collect()
	}
}
