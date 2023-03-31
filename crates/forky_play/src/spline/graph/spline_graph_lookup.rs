use super::*;
use crate::{spline::Spline, *};
use bevy::{prelude::*, utils::HashMap};
use derive_deref::{Deref, DerefMut};
use forky_core::*;
use petgraph::{
	graphmap::{Edges, UnGraphMap},
	Undirected,
};

#[derive(Component, Deref, DerefMut)]
pub struct SplineGraphId(pub u64);

#[derive(Resource, Default)]
pub struct SplineGraphLookup {
	next_graph_id: u64,
	graphs: HashMap<u64, SplineGraph>,
}

impl SplineGraphLookup {
	pub fn add(&mut self, graph: SplineGraph) -> SplineGraphId {
		self.graphs.insert(self.next_graph_id, graph);
		self.next_graph_id += 1;
		SplineGraphId(self.next_graph_id - 1)
	}
}

impl std::ops::Deref for SplineGraphLookup {
	type Target = HashMap<u64, SplineGraph>;
	fn deref(&self) -> &Self::Target { &self.graphs }
}

impl std::ops::DerefMut for SplineGraphLookup {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graphs }
}
