use crate::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Default)]
pub struct NodeSystemConfig {
	pub apply_deferred: bool,
}



pub trait IntoNodeSystem:
	'static + std::fmt::Debug + Default + Clone + Send + Sync
{
	fn add_node_system<Node: AiNode>(
		&self,
		schedule: &mut Schedule,
		set: impl SystemSet,
		config: &NodeSystemConfig,
	);
}
