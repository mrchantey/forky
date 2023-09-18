use crate::*;
use bevy::prelude::*;

pub trait AddChoiceSystem: 'static + Clone + Send + Sync {
	fn add_choice_system<C: Choice>(&self, app: &mut App, set: impl SystemSet);
}

pub trait ChoiceSystems: 'static + Send + Sync + Clone {
	type EdgeSystem: AddChoiceSystem;
	type NodeSystem: AddChoiceSystem;
	fn get_edge(&self) -> Self::EdgeSystem;
	fn get_node(&self) -> Self::NodeSystem;

	fn add_choice_systems<C: Choice>(
		&self,
		app: &mut App,
		sets: &impl NodeSets,
	) {
		self.get_edge()
			.add_choice_system::<C>(app, sets.child_edge_set());
		self.get_node()
			.add_choice_system::<C>(app, sets.child_node_set());
	}
}

//doesnt work?
impl<BF, BA, EdgeSystem, NodeSystem> ChoiceSystems for (BF, BA)
where
	BF: 'static + Clone + Send + Sync + Fn() -> EdgeSystem,
	BA: 'static + Clone + Send + Sync + Fn() -> NodeSystem,
	EdgeSystem: AddChoiceSystem,
	NodeSystem: AddChoiceSystem,
{
	type EdgeSystem = EdgeSystem;
	type NodeSystem = NodeSystem;
	fn get_edge(&self) -> Self::EdgeSystem { (self.0)() }
	fn get_node(&self) -> Self::NodeSystem { (self.1)() }
}

#[derive(Clone)]
pub struct ChoiceBuilder<EdgeSystem, NodeSystem>
where
	EdgeSystem: AddChoiceSystem,
	NodeSystem: AddChoiceSystem,
{
	pub edge: fn() -> EdgeSystem,
	pub node: fn() -> NodeSystem,
}

impl<EdgeSystem, NodeSystem> ChoiceBuilder<EdgeSystem, NodeSystem>
where
	EdgeSystem: AddChoiceSystem,
	NodeSystem: AddChoiceSystem,
{
	pub fn new(edge: fn() -> EdgeSystem, node: fn() -> NodeSystem) -> Self {
		Self { edge, node }
	}
}

impl<EdgeSystem, NodeSystem> ChoiceSystems
	for ChoiceBuilder<EdgeSystem, NodeSystem>
where
	EdgeSystem: AddChoiceSystem,
	NodeSystem: AddChoiceSystem,
{
	type EdgeSystem = EdgeSystem;
	type NodeSystem = NodeSystem;
	fn get_edge(&self) -> Self::EdgeSystem { (self.edge)() }
	fn get_node(&self) -> Self::NodeSystem { (self.node)() }
}
