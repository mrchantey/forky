use crate::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Default)]
pub struct NodeSystemConfig {
	//todo deprecate
	pub apply_deferred: bool,
}



pub trait IntoNodeSystem:
	'static + Default + Clone + Send + Sync
{
	fn add_node_system<Node: AiNode>(
		&self,
		schedule: &mut Schedule,
		set: impl SystemSet,
		config: &NodeSystemConfig,
	);
}


impl<T> IntoNodeSystem for T
where
	// M: 'static,
	T: 'static
		+ Default
		+ Sized
		+ Send
		+ Clone
		+ Sync
		+ IntoSystemConfigs<Self>,
{
	// type Marker = M;
	fn add_node_system<Node: AiNode>(
		&self,
		schedule: &mut Schedule,
		set: impl SystemSet,
		_config: &NodeSystemConfig,
	) {
		schedule.add_systems(self.clone().in_set(set));
	}
}
