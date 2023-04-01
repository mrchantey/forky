use super::*;

use bevy::{prelude::*, utils::HashMap};
use derive_deref::{Deref, DerefMut};



#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SplineGraphId(pub u64);

#[derive(Resource, Default)]
pub struct SplineGraphLookup {
	next_graph_id: u64,
	graphs: HashMap<u64, SplineGraph>,
}

impl SplineGraphLookup {
	pub fn create_graph(&mut self) -> (SplineGraphId, &mut SplineGraph) {
		self.graphs.insert(self.next_graph_id, SplineGraph::new());
		let graph = self.graphs.get_mut(&self.next_graph_id).unwrap();
		let graph_id = SplineGraphId(self.next_graph_id);
		self.next_graph_id += 1;
		(graph_id, graph)
	}
}

impl std::ops::Deref for SplineGraphLookup {
	type Target = HashMap<u64, SplineGraph>;
	fn deref(&self) -> &Self::Target { &self.graphs }
}

impl std::ops::DerefMut for SplineGraphLookup {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graphs }
}
