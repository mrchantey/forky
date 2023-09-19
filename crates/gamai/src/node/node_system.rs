use crate::*;
use bevy_ecs::prelude::*;

pub trait IntoNodeSystem:
	'static + std::fmt::Debug + Default + Clone + Send + Sync
{
	fn add_node_system<A: AiNode>(schedule: &mut Schedule, set: impl SystemSet);
}
