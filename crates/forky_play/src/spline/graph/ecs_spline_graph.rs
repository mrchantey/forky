use super::*;
use crate::{
	materials::{RenderBundle, UvMaterial},
	spline::*,
	*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Debug, Clone)]
pub struct EcsSplineGraph {
	pub id: u64,
	pub graph: SplineGraph,
	pub edge_material: Handle<UvMaterial>,
	pub edge_subdivisions: usize,
	pub edges: IdHashMap<EcsSplineEdge>,
}

impl EcsSplineGraph {
	pub fn new(id: u64, edge_material: Handle<UvMaterial>) -> Self {
		Self {
			id,
			edge_material,
			edge_subdivisions: 32,
			graph: SplineGraph::new(),
			edges: IdHashMap::<EcsSplineEdge>::new(),
		}
	}

	pub fn create_point(
		&mut self,
		commands: &mut Commands,
		interaction_settings: &Res<tool::InteractionSettings>,
		interaction_resources: &Res<tool::InteractionResources>,
		point_index: u32,
		position: Vec3,
	) -> Entity {
		commands
			.spawn((
				TransformBundle::from(Transform::from_translation(position)),
				materials::RenderBundle {
					material: interaction_resources.inactive_material.clone(),
					mesh: interaction_resources.node_mesh.clone(),
					..Default::default()
				},
				Collider::ball(interaction_settings.node_radius),
				tool::Interactable,
				EcsSplineGraphId(self.id),
				SplinePointIndex(point_index),
			))
			.id()
	}

	pub fn create_node(
		&mut self,
		commands: &mut Commands,
		entity: Entity,
	) -> SplineNode {
		let node = self.graph.create_node();
		commands.entity(entity).insert(node);
		node
	}

	pub fn create_points(
		&mut self,
		commands: &mut Commands,
		interaction_settings: &Res<tool::InteractionSettings>,
		interaction_resources: &Res<tool::InteractionResources>,
		spline: Spline,
	) -> (SplineNode, SplineNode, Vec<Entity>) {
		let mut first = SplineNode(0);
		let mut last = SplineNode(0);
		let spline_points = spline.get_points();
		let mut points: Vec<Entity> = Vec::with_capacity(spline_points.len());

		for (point_index, position) in spline_points.iter().enumerate() {
			let point = self.create_point(
				commands,
				interaction_settings,
				interaction_resources,
				point_index as u32,
				*position,
			);
			if point_index == 0 {
				first = self.create_node(commands, point);
			} else if point_index == spline.get_points().len() - 1 {
				last = self.create_node(commands, point);
			}
			points.push(point);
		}
		(first, last, points)
	}

	pub fn create_edge_from_spline(
		&mut self,
		commands: &mut Commands,
		interaction_settings: Res<tool::InteractionSettings>,
		interaction_resources: Res<tool::InteractionResources>,
		spline: Spline,
	) {
		let (node1, node2, points) = self.create_points(
			commands,
			&interaction_settings,
			&interaction_resources,
			spline.clone(),
		);

		let (edge_id, _) = self.create_edge(commands, node1, node2, spline);

		for point in points.iter() {
			commands
				.entity(*point)
				.insert(SplineEdgeList(vec![edge_id]));
		}
	}

	pub fn create_edge(
		&mut self,
		commands: &mut Commands,
		node1: SplineNode,
		node2: SplineNode,
		spline: Spline,
	) -> (u64, &mut EcsSplineEdge) {
		self.graph.create_edge(node1, node2, spline);

		let entity = commands
			.spawn((
				TransformBundle::default(),
				RenderBundle::new(
					Handle::<Mesh>::default(),
					self.edge_material.clone(),
				),
				spline,
			))
			.id();
		self.edges.insert_next(EcsSplineEdge {
			link: SplineLink::new(node1, node2),
			mesh: entity,
			points: Vec::new(), //TODO points
		})
	}

	// pub fn update_edge_from_node(&mut self, node: &SplineNode, position: Vec3) {
	// 	self.graph.edges(*node).for_each(|(_, _, edge)| {
	// 		let mut edge = edge.clone();
	// 		if edge.a == *node {
	// 			edge.spline.set_first(position);
	// 		} else {
	// 			edge.spline.set_last(position);
	// 		}
	// 		self.graph.update_edge(edge.a, edge.b, edge);
	// 	});
	// }

	pub fn update_edge_from_point(
		&mut self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		point_index: SplinePointIndex,
		position: Vec3,
		edges: &SplineEdgeList,
	) {
		for ecs_edge in edges.iter().map(|edge| self.edges.get(edge).unwrap()) {
			let edge = self
				.graph
				.edge_weight_mut(ecs_edge.link.a, ecs_edge.link.b)
				.unwrap();
			edge.spline.set_point(position, *point_index).unwrap();
			commands.entity(ecs_edge.mesh).insert(meshes.add(
				mesh::append_spline_mesh(&edge.spline, self.edge_subdivisions),
			));
		}
	}
}
