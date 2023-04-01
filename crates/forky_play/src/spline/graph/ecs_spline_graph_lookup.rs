use super::*;
use crate::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct EcsSplineGraphId(pub u64);

#[derive(Resource, Deref, DerefMut)]
pub struct EcsSplineGraphLookup(pub IdHashMap<EcsSplineGraph>);

impl EcsSplineGraphLookup {
	pub fn new() -> Self { Self(IdHashMap::<EcsSplineGraph>::new()) }

	pub fn create_graph(
		&mut self,
		material: Handle<materials::UvMaterial>,
	) -> (SplineGraphId, &mut EcsSplineGraph) {
		let id = self.0.next_id;
		let (id, graph) = self.insert_next(EcsSplineGraph::new(id, material));
		(SplineGraphId(id), graph)
	}
}
