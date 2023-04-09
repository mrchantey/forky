use super::*;
use crate::*;
use bevy::prelude::*;

#[derive(Component, Debug, Deref, DerefMut, Clone, Copy)]
pub struct EcsSplineGraphId(pub u64);

#[derive(Resource, Deref, DerefMut)]
pub struct EcsSplineGraphLookup(pub IdHashMap<EcsSplineGraph>);

impl EcsSplineGraphLookup {
	pub fn new() -> Self { Self(IdHashMap::<EcsSplineGraph>::new()) }

	pub fn create_graph(
		&mut self,
		material: Handle<materials::UvMaterial>,
	) -> &mut EcsSplineGraph {
		self.create_graph_with_options(material, false)
	}
	pub fn create_graph_with_options(
		&mut self,
		material: Handle<materials::UvMaterial>,
		create_handles: bool,
	) -> &mut EcsSplineGraph {
		let id = EcsSplineGraphId(self.0.next_id);
		self.insert_next(EcsSplineGraph::new(id, material, create_handles))
			.1
	}
}
