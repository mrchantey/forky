use super::*;
use crate::IdHashMap;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SplineGraphId(pub u64);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SplineGraphLookup(pub IdHashMap<SplineGraph>);

impl SplineGraphLookup {
	pub fn new() -> Self { Self::default() }

	pub fn create_graph(&mut self) -> (SplineGraphId, &mut SplineGraph) {
		let (id, graph) = self.insert_next(SplineGraph::new());
		(SplineGraphId(id), graph)
	}
}
