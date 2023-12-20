use crate::prelude::*;
use anyhow::Result;
use bevy_ecs::schedule::SystemConfigs;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityWorldMut;

#[typetag::serde]
pub trait Action: 'static {
	fn duplicate(&self) -> Box<dyn Action>;

	fn spawn(&self, entity: &mut EntityWorldMut<'_>);
	fn spawn_with_command(&self, entity: &mut EntityCommands);

	// fn pre_tick_system(&self) -> SystemConfigs;
	fn tick_system(&self) -> SystemConfigs;
	fn post_tick_system(&self) -> SystemConfigs;

	fn meta(&self) -> ActionMeta;
}

impl<T: Action> IntoTree<ActionList> for T {
	fn into_tree(self) -> Tree<ActionList> { Tree::new(vec![Box::new(self)]) }
	fn with_child(self, child: impl IntoTree<ActionList>) -> Tree<ActionList> {
		self.into_tree().with_child(child)
	}
	fn with_leaf(self, child: ActionList) -> Tree<ActionList> {
		self.into_tree().with_leaf(child)
	}
}

pub type SetActionFunc = Box<dyn Fn(&mut EntityCommands) -> Result<()>>;

pub trait SetAction: Action {
	fn set(&mut self, func: SetActionFunc);
}

#[derive(Debug, Clone)]
pub struct ActionMeta {
	pub name: &'static str,
	pub id: usize,
}
