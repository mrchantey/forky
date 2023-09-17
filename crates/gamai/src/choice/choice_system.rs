use crate::*;
use bevy::prelude::*;

pub trait AddChoiceSystem: 'static + Clone + Send + Sync {
	fn add_choice_system<C: Choice>(&self, app: &mut App, set: impl SystemSet);
}

pub trait ChoiceSystems: 'static + Send + Sync + Clone {
	type Factor: AddChoiceSystem;
	type Action: AddChoiceSystem;
	fn get_factor(&self) -> Self::Factor;
	fn get_action(&self) -> Self::Action;

	fn add_choice_systems<C: Choice>(
		&self,
		app: &mut App,
		sets: &impl SolverSets,
	) {
		self.get_factor()
			.add_choice_system::<C>(app, sets.factor_set());
		self.get_action()
			.add_choice_system::<C>(app, sets.action_set());
	}
}

//doesnt work?
impl<BF, BA, Factor, Action> ChoiceSystems for (BF, BA)
where
	BF: 'static + Clone + Send + Sync + Fn() -> Factor,
	BA: 'static + Clone + Send + Sync + Fn() -> Action,
	Factor: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	type Factor = Factor;
	type Action = Action;
	fn get_factor(&self) -> Self::Factor { (self.0)() }
	fn get_action(&self) -> Self::Action { (self.1)() }
}

#[derive(Clone)]
pub struct ChoiceBuilder<Factor, Action>
where
	Factor: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	pub factor: fn() -> Factor,
	pub action: fn() -> Action,
}

impl<Factor, Action> ChoiceBuilder<Factor, Action>
where
	Factor: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	pub fn new(factor: fn() -> Factor, action: fn() -> Action) -> Self {
		Self { factor, action }
	}
}

impl<Factor, Action> ChoiceSystems for ChoiceBuilder<Factor, Action>
where
	Factor: AddChoiceSystem,
	Action: AddChoiceSystem,
{
	type Factor = Factor;
	type Action = Action;
	fn get_factor(&self) -> Self::Factor { (self.factor)() }
	fn get_action(&self) -> Self::Action { (self.action)() }
}
