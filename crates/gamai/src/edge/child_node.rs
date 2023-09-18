use crate::*;
use bevy::prelude::*;

pub trait ChildNodeSystems: 'static + Send + Sync + Clone {
	type EdgeSystem: EdgeSystemBuilder;
	type NodeSystem: EdgeSystemBuilder;
	fn edge_system(&self) -> Self::EdgeSystem;
	fn node_system(&self) -> Self::NodeSystem;

	fn add_edge_systems<C: AiEdge>(&self, app: &mut App, sets: &impl NodeSets) {
		self.edge_system()
			.add_edge_system::<C>(app, sets.child_edge_set());
		self.node_system()
			.add_edge_system::<C>(app, sets.child_node_set());
	}
}

//doesnt work?
impl<BF, BA, EdgeSystem, NodeSystem> ChildNodeSystems for (BF, BA)
where
	BF: 'static + Clone + Send + Sync + Fn() -> EdgeSystem,
	BA: 'static + Clone + Send + Sync + Fn() -> NodeSystem,
	EdgeSystem: EdgeSystemBuilder,
	NodeSystem: EdgeSystemBuilder,
{
	type EdgeSystem = EdgeSystem;
	type NodeSystem = NodeSystem;
	fn edge_system(&self) -> Self::EdgeSystem { (self.0)() }
	fn node_system(&self) -> Self::NodeSystem { (self.1)() }
}

#[derive(Clone)]
pub struct EdgeBuilder<EdgeSystem, NodeSystem>
where
	EdgeSystem: EdgeSystemBuilder,
	NodeSystem: EdgeSystemBuilder,
{
	pub edge: fn() -> EdgeSystem,
	pub node: fn() -> NodeSystem,
}

impl<EdgeSystem, NodeSystem> EdgeBuilder<EdgeSystem, NodeSystem>
where
	EdgeSystem: EdgeSystemBuilder,
	NodeSystem: EdgeSystemBuilder,
{
	pub fn new(edge: fn() -> EdgeSystem, node: fn() -> NodeSystem) -> Self {
		Self { edge, node }
	}
}

impl<EdgeSystem, NodeSystem> ChildNodeSystems
	for EdgeBuilder<EdgeSystem, NodeSystem>
where
	EdgeSystem: EdgeSystemBuilder,
	NodeSystem: EdgeSystemBuilder,
{
	type EdgeSystem = EdgeSystem;
	type NodeSystem = NodeSystem;
	fn edge_system(&self) -> Self::EdgeSystem { (self.edge)() }
	fn node_system(&self) -> Self::NodeSystem { (self.node)() }
}
