use crate::*;
use bevy::prelude::*;

pub trait AddChoiceSystem: 'static + Clone + Send + Sync {
	fn add_choice_system<C: Choice>(&self, app: &mut App, set: impl SystemSet);
}

pub trait ChoiceSystems: 'static + Send + Sync + Clone {
	type EdgeSystem: AddChoiceSystem;
	type Action: AddChoiceSystem;
	fn get_edge(&self) -> Self::EdgeSystem;
	fn get_action(&self) -> Self::Action;

	fn add_choice_systems<C: Choice>(
		&self,
		app: &mut App,
		sets: &impl SolverSets,
	) {
		self.get_edge().add_choice_system::<C>(app, sets.edge_set());
		self.get_action()
			.add_choice_system::<C>(app, sets.action_set());
	}
}

//doesnt work?
impl<BF, BA, EdgeSystem, Action> ChoiceSystems for (BF, BA)
where
	BF: 'static + Clone + Send + Sync + Fn() -> EdgeSystem,
	BA: 'static + Clone + Send + Sync + Fn() -> Action,
	EdgeSystem: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	type EdgeSystem = EdgeSystem;
	type Action = Action;
	fn get_edge(&self) -> Self::EdgeSystem { (self.0)() }
	fn get_action(&self) -> Self::Action { (self.1)() }
}

#[derive(Clone)]
pub struct ChoiceBuilder<EdgeSystem, Action>
where
	EdgeSystem: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	pub edge: fn() -> EdgeSystem,
	pub action: fn() -> Action,
}

impl<EdgeSystem, Action> ChoiceBuilder<EdgeSystem, Action>
where
	EdgeSystem: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	pub fn new(edge: fn() -> EdgeSystem, action: fn() -> Action) -> Self {
		Self { edge, action }
	}
}

impl<EdgeSystem, Action> ChoiceSystems for ChoiceBuilder<EdgeSystem, Action>
where
	EdgeSystem: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	type EdgeSystem = EdgeSystem;
	type Action = Action;
	fn get_edge(&self) -> Self::EdgeSystem { (self.edge)() }
	fn get_action(&self) -> Self::Action { (self.action)() }
}
