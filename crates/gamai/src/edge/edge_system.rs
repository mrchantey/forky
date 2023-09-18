use crate::*;
use bevy::prelude::*;

pub trait EdgeSystemBuilder: 'static + Clone + Send + Sync {
	fn add_edge_system<C: AiEdge>(&self, app: &mut App, set: impl SystemSet);
}
