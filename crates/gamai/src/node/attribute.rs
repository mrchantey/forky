// use crate::*;
use crate::*;
use bevy_ecs::prelude::*;
// use bevy_ecs::schedule::SystemConfigs;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Attributes<
	PreParentUpdate: IntoAction,
	PreUpdate: IntoAction,
	Update: IntoAction,
	PostUpdate: IntoAction,
> {
	pub pre_parent_update: PreParentUpdate,
	pub pre_update: PreUpdate,
	pub update: Update,
	pub update_apply_deferred: bool,
	pub post_update: PostUpdate,
}


impl<
		PreParentUpdate: IntoAction,
		PreUpdate: IntoAction,
		Update: IntoAction,
		PostUpdate: IntoAction,
	> Attributes<PreParentUpdate, PreUpdate, Update, PostUpdate>
{
	pub fn new(
		pre_parent_update: PreParentUpdate,
		pre_update: PreUpdate,
		update: Update,
		update_apply_deferred: bool,
		post_update: PostUpdate,
	) -> Self {
		Self {
			pre_parent_update,
			pre_update,
			update,
			update_apply_deferred,
			post_update,
		}
	}
}

pub type DefaultAttributes =
	Attributes<EmptyAction, EmptyAction, EmptyAction, EmptyAction>;

impl Default for DefaultAttributes {
	fn default() -> Self {
		Self {
			pre_parent_update: EmptyAction::default(),
			pre_update: EmptyAction::default(),
			update_apply_deferred: false,
			update: EmptyAction::default(),
			post_update: EmptyAction::default(),
		}
	}
}
impl<
		PreParentUpdate: IntoAction,
		PreUpdate: IntoAction,
		Update: IntoAction,
		PostUpdate: IntoAction,
	> IntoAction for Attributes<PreParentUpdate, PreUpdate, Update, PostUpdate>
{
	fn into_action_configs<Node: AiNode>(
		self,
	) -> bevy_ecs::schedule::SystemConfigs {
		let self_update = self
			.update
			.into_action_configs::<Node>()
			.in_set(Node::update_set());

		(
			self.pre_parent_update
				.into_action_configs::<Node>()
				.in_set(Node::pre_parent_update_set()),
			self.pre_update
				.into_action_configs::<Node>()
				.in_set(Node::pre_update_set()),
			if self.update_apply_deferred {
				// println!("applying deferred");
				(self_update, apply_deferred).chain()
			} else {
				self_update
			}
			.in_set(Node::update_set()),
			self.post_update
				.into_action_configs::<Node>()
				.in_set(Node::post_update_set()),
			remove_running
				.into_action_configs::<Node>()
				.in_set(Node::post_update_set()),
		)
			.into_configs()
	}
}
