use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;

#[derive(Debug, PartialEq, Deref, DerefMut, Component)]
pub struct TargetEntity(pub Entity);

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PreTickSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct TickSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PostTickSet;
