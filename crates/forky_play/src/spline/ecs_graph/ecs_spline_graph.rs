use super::*;
use crate::{
	materials::UvMaterial,
	spline::{graph::*, *},
};
use bevy::{prelude::*, utils::HashMap};
use petgraph::data::Build;

#[derive(Debug, Clone)]
pub struct EcsSplineGraph {
	pub id: EcsSplineGraphId,
	pub graph: SplineGraph,
	edge_material: Handle<UvMaterial>,
	edge_subdivisions: usize,
	nodes: HashMap<SplineNode, EcsSplineNode>,
	edges: HashMap<SplineEdgeId, EcsSplineEdge>,
}

impl EcsSplineGraph {
	pub fn new(
		id: EcsSplineGraphId,
		edge_material: Handle<UvMaterial>,
	) -> Self {
		Self {
			id,
			edge_material,
			edge_subdivisions: 32,
			graph: Default::default(),
			edges: Default::default(),
			nodes: Default::default(),
		}
	}

	pub fn create_node(
		&mut self,
		commands: &mut Commands,
		position: Vec3,
	) -> EcsSplineNode {
		let node = self.graph.create_node(position);
		let entity = commands
			.spawn(EcsSplineNodeBundle::new(position, node, self.id))
			.id();
		let ecs_node = EcsSplineNode { entity, node };
		self.nodes.insert(node, ecs_node);
		ecs_node
	}

	pub fn remove_node(&mut self, commands: &mut Commands, node: SplineNode) {
		for edge in self.graph.clone_edges(node) {
			self.remove_edge(commands, edge);
		}
		commands.entity(self.nodes[&node].entity).despawn();
	}


	pub fn create_edge(
		&mut self,
		commands: &mut Commands,
		node1: SplineNode,
		node2: SplineNode,
	) -> EcsSplineEdge {
		let pos1 = self.graph.positions[&node1];
		let pos2 = self.graph.positions[&node2];
		let spline = Spline::Linear(LinearSpline::new(pos1, pos2));
		self.create_edge_with_spline(commands, node1, node2, spline)
	}

	pub fn create_edge_from_spline(
		&mut self,
		commands: &mut Commands,
		spline: Spline,
	) -> EcsSplineEdge {
		let node1 = self.create_node(commands, spline.first());
		let node2 = self.create_node(commands, spline.last());
		self.create_edge_with_spline(commands, node1.node, node2.node, spline)
	}

	pub fn create_edge_with_spline(
		&mut self,
		commands: &mut Commands,
		node1: SplineNode,
		node2: SplineNode,
		spline: Spline,
	) -> EcsSplineEdge {
		let edge = self.graph.create_edge(node1, node2, spline);
		let mesh = commands
			.spawn(SplineEdgeBundle::new(
				edge,
				self.edge_material.clone(),
				self.id,
			))
			.id();

		let handles = spline
			.get_points()
			.iter()
			.enumerate()
			.skip(1)
			.rev()
			.skip(1)
			.map(|(index, position)| {
				self.create_handle(
					commands,
					*position,
					SplineHandleIndex(index as u32),
					edge,
				)
			})
			.collect::<Vec<_>>();

		let ecs_edge = EcsSplineEdge {
			node1,
			node2,
			mesh,
			handles,
		};
		self.edges.insert(edge.id, ecs_edge.clone());

		ecs_edge
	}

	pub fn create_handle(
		&mut self,
		commands: &mut Commands,
		position: Vec3,
		handle_index: SplineHandleIndex,
		edge: SplineEdge,
	) -> Entity {
		commands
			.spawn(EcsSplineHandleBundle::new(
				position,
				handle_index,
				edge.id,
				self.id,
			))
			.id()
	}

	pub fn remove_edge(&mut self, commands: &mut Commands, edge: SplineEdge) {
		self.graph.remove_edge(edge.a, edge.b);
		commands.entity(self.edges[&edge.id].mesh).despawn();
		for handle in self.edges[&edge.id].handles.iter() {
			commands.entity(*handle).despawn();
		}
		if self.graph.edges(edge.a).count() == 0 {
			self.remove_node(commands, edge.a);
		}
		if self.graph.edges(edge.b).count() == 0 {
			self.remove_node(commands, edge.b);
		}
	}

	pub fn update_edge_mesh(
		&mut self,
		commands: &mut Commands,
		edge_id: &SplineEdgeId,
		meshes: &mut ResMut<Assets<Mesh>>,
	) {
		let ecs_edge = self.edges.get(edge_id).unwrap();
		commands.entity(ecs_edge.mesh).insert(
			meshes.add(mesh::create_spline_mesh(
				&self
					.graph
					.edge_weight(ecs_edge.node1, ecs_edge.node2)
					.unwrap()
					.spline,
				self.edge_subdivisions,
			)),
		);
	}

	pub fn update_edge_from_handle(
		&mut self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		edge_id: &SplineEdgeId,
		position: Vec3,
		handle_index: &SplineHandleIndex,
	) {
		//TODO this should be after updating edge?
		self.update_edge_mesh(commands, edge_id, meshes);
		let ecs_edge = self.edges.get_mut(&edge_id).unwrap();
		self.graph
			.edge_weight_mut(ecs_edge.node1, ecs_edge.node2)
			.unwrap()
			.spline
			.set_point(position, **handle_index)
			.unwrap();
	}

	pub fn update_node_position(
		&mut self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		node: &SplineNode,
		position: Vec3,
	) {
		self.graph.positions.insert(*node, position);
		//TODO can this be optimized, ie edge_weight_mut?
		for edge in self.graph.clone_edges(*node).iter() {
			self.update_edge_mesh(commands, &edge.id, meshes);
			let mut edge = edge.clone();

			if edge.a == *node {
				edge.spline.set_first(position);
			} else {
				edge.spline.set_last(position);
			}
			self.graph.update_edge(edge.a, edge.b, edge);
		}
		//TODO we dont need to update the entire graph
		self.apply_catmull_rom(commands, meshes);
	}

	pub fn apply_catmull_rom(
		&mut self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
	) {
		let solved = self.graph.solve_catmull_rom();
		for (a, b, mut edge) in self
			.graph
			.all_edges()
			.map(|(a, b, e)| (a, b, e.clone()))
			.collect::<Vec<_>>()
		{
			if let Some(handle_ab) = solved.get(&(a, b)) {
				if let Some(handle_ba) = solved.get(&(b, a)) {
					edge.spline = Spline::Cubic(CubicSpline {
						p0: self.graph.positions[&a],
						p1: *handle_ab,
						p2: *handle_ba,
						p3: self.graph.positions[&b],
					});
					self.graph.update_edge(a, b, edge);
					self.update_edge_mesh(commands, &edge.id, meshes);
				} else {
					panic!("partially solved graph for {a},{b}");
				}
			} else {
				panic!("unsolved solved graph for {a},{b}");
			}

			// edge.spline = solved[&edge.id].clone();
		}
		// for (edge, position) in solved.iter() {
		// 	self.update_node_position(commands, meshes, &edge.1, *position);
		// }
	}
}
