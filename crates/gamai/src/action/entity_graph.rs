use crate::prelude::*;
use anyhow::anyhow;
use anyhow::Result;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct EntityGraph(pub DiGraph<Entity, ()>);

impl EntityGraph {
	pub fn set_action(
		&self,
		world: &mut World,
		message: &SetActionMessage,
	) -> Result<()> {
		let mut entity = self
			.0
			.node_weight(NodeIndex::new(*message.index))
			.map(|entity| world.entity_mut(*entity))
			.ok_or_else(|| anyhow!("Node not found: {}", *message.index))?;

		message.value.spawn(&mut entity);
		Ok(())
	}
	pub fn into_tree(self) -> Tree<Entity> { self.0.into_tree() }
}
