use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;


/// Outgoing edges of an action, aka Children.
#[derive(Debug, Default, Clone, Deref, DerefMut, Component)]
pub struct Edges(pub Vec<Entity>);


impl Edges {
	pub fn new() -> Self { Self(Vec::new()) }

	pub fn with_child(mut self, edge: Entity) -> Self {
		self.push(edge);
		self
	}
}
