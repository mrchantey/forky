// use crate::*;
use crate::*;
use bevy_ecs::prelude::*;
// use bevy_ecs::schedule::SystemConfigs;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Attributes<
	PreParentUpdate: IntoNodeSystem,
	PreUpdate: IntoNodeSystem,
	Update: IntoNodeSystem,
	PostUpdate: IntoNodeSystem,
> {
	pub pre_parent_update: PreParentUpdate,
	pub pre_update: PreUpdate,
	pub update: Update,
	pub update_apply_deferred: bool,
	pub post_update: PostUpdate,
}

impl<
		PreParentUpdate: IntoNodeSystem,
		PreUpdate: IntoNodeSystem,
		Update: IntoNodeSystem,
		PostUpdate: IntoNodeSystem,
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

pub type DefaultAttributes = Attributes<
	EmptyNodeSystem,
	EmptyNodeSystem,
	EmptyNodeSystem,
	EmptyNodeSystem,
>;

impl Default for DefaultAttributes {
	fn default() -> Self {
		Self {
			pre_parent_update: EmptyNodeSystem::default(),
			pre_update: EmptyNodeSystem::default(),
			update_apply_deferred: false,
			update: EmptyNodeSystem::default(),
			post_update: EmptyNodeSystem::default(),
		}
	}
}
impl<
		PreParentUpdate: IntoNodeSystem,
		PreUpdate: IntoNodeSystem,
		Update: IntoNodeSystem,
		PostUpdate: IntoNodeSystem,
	> IntoNodeSystem
	for Attributes<PreParentUpdate, PreUpdate, Update, PostUpdate>
{
	fn into_node_system_configs<Node: AiNode>(
		self,
	) -> bevy_ecs::schedule::SystemConfigs {
		let mut systems = (
			self.pre_parent_update
				.into_node_system_configs::<Node>()
				.in_set(Node::pre_parent_update_set()),
			self.pre_update
				.into_node_system_configs::<Node>()
				.in_set(Node::pre_update_set()),
			self.update
				.into_node_system_configs::<Node>()
				.in_set(Node::update_set()),
			self.post_update
				.into_node_system_configs::<Node>()
				.in_set(Node::post_update_set()),
		)
			.into_configs();

		if self.update_apply_deferred {
			systems = (
				systems,
				apply_deferred
					.after(Node::update_set())
					.before(Node::post_update_set()),
			)
				.into_configs();
		}

		systems
	}
}
