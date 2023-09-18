use crate::*;
use bevy_ecs::prelude::*;

pub trait EdgeSystemBuilder: 'static + Clone + Send + Sync {
	fn add_edge_system<C: AiEdge>(
		&self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	);
}
