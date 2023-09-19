use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;
use anyhow::Result;

pub trait IntoNodeSystem:
	'static + std::fmt::Debug + Default + Clone + Send + Sync
{
	fn add_node_system<A: AiNode>(schedule: &mut Schedule, set: impl SystemSet);
}
