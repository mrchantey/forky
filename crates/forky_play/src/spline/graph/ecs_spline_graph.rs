use super::*;
use crate::{
	materials::{RenderBundle, UvMaterial},
	spline::*,
	*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use petgraph::data::Build;

#[derive(Debug, Clone)]
pub struct EcsSplineGraph {
	pub id: u64,
	pub graph: SplineGraph,
	pub edge_material: Handle<UvMaterial>,
	pub edge_subdivisions: usize,
	pub nodes: IdHashMap<Entity>,
	pub edges: IdHashMap<Entity>,
}

impl EcsSplineGraph {
	pub fn new(id: u64, edge_material: Handle<UvMaterial>) -> Self {
		Self {
			id,
			edge_material,
			edge_subdivisions: 10,
			graph: SplineGraph::new(),
			nodes: IdHashMap::<Entity>::new(),
			edges: IdHashMap::<Entity>::new(),
		}
	}
	pub fn create_node(
		&mut self,
		commands: &mut Commands,
		interaction_settings: &Res<tool::InteractionSettings>,
		interaction_resources: &Res<tool::InteractionResources>,
		position: Vec3,
	) -> SplineNode {
		let node = self.graph.create_node();
		let entity = commands
			.spawn((
				SplineNodeBundle::new(position, node),
				materials::RenderBundle {
					material: interaction_resources.inactive_material.clone(),
					mesh: interaction_resources.node_mesh.clone(),
					..Default::default()
				},
				Collider::ball(interaction_settings.node_radius),
				tool::Interactable,
				EcsSplineGraphId(self.id),
			))
			.id();
		self.nodes.insert(*node, entity);
		node
	}

	pub fn create_edge_from_spline(
		&mut self,
		commands: &mut Commands,
		interaction_settings: Res<tool::InteractionSettings>,
		interaction_resources: Res<tool::InteractionResources>,
		spline: Spline,
	) {
		let node1 = self.create_node(
			commands,
			&interaction_settings,
			&interaction_resources,
			spline.position(0.),
		);
		let node2 = self.create_node(
			commands,
			&interaction_settings,
			&interaction_resources,
			spline.position(1.),
		);

		let edge = self.graph.create_edge(node1, node2, spline);

		let entity = commands.spawn((
			TransformBundle::default(),
			RenderBundle::new(
				Handle::<Mesh>::default(),
				self.edge_material.clone(),
			),
			spline,
		));
		self.edges.insert(edge.id, entity.id());
	}


	pub fn edge_entities(
		&self,
		node: &SplineNode,
	) -> impl Iterator<Item = (&Entity, &SplineEdge)> {
		self.graph
			.edges(*node)
			.map(|(_, _, edge)| (self.edges.get(&edge.id).unwrap(), edge))
		// self.edges.get(&edge)
	}

	pub fn update_node_position(&mut self, node: &SplineNode, position: Vec3) {
		let edges = self
			.graph
			.edges(*node)
			.map(|(_, _, edge)| {
				let mut edge = edge.clone();
				if edge.a == *node {
					edge.spline.set_first(position);
				} else {
					edge.spline.set_last(position);
				}
				edge
			})
			.collect::<Vec<_>>();

		for edge in edges {
			self.graph.update_edge(edge.a, edge.b, edge);
		}
	}
}
