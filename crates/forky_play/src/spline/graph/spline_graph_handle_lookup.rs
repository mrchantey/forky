use super::*;
use crate::{spline::Spline, *};
use bevy::{prelude::*, utils::HashMap};
use derive_deref::{Deref, DerefMut};
use forky_core::*;

#[derive(Resource, Default)]
pub struct SplineGraphHandleLookup {
	next_graph_id: u64,
	nodes: Vec<Entity>,
	edges: Vec<Entity>,
}

impl SplineGraphHandleLookup {
	pub fn create_graph(&mut self) -> (SplineGraphId, &mut SplineGraph) {
		self.graphs.insert(self.next_graph_id, SplineGraph::new());
		let graph = self.graphs.get_mut(&self.next_graph_id).unwrap();
		let graph_id = SplineGraphId(self.next_graph_id);
		self.next_graph_id += 1;
		(graph_id, graph)
	}
}

// impl std::ops::Deref for SplineGraphLookup {
// 	type Target = HashMap<u64, SplineGraph>;
// 	fn deref(&self) -> &Self::Target { &self.graphs }
// }

// impl std::ops::DerefMut for SplineGraphLookup {
// 	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graphs }
// }
