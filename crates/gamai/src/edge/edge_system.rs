use crate::*;
use bevy::prelude::*;

pub trait EdgeSystemBuilder: 'static + Clone + Send + Sync {
	fn add_edge_system<C: Edge>(&self, app: &mut App, set: impl SystemSet);
}
