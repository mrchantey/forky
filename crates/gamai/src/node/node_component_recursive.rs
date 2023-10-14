use crate::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct NodeComponentRecursive<'a, T: IntoNodeComponent> {
	pub depth: usize,
	pub value: Option<&'a T>,
	pub children: Vec<NodeComponentRecursive<'a, T>>,
}

impl<'a, T: IntoNodeComponent> NodeComponentRecursive<'a, T> {
	pub fn new<M, N: AiNode>(
		node: impl IntoNode<M, Out = N>,
		world: &World,
		entity: Entity,
	) -> NodeComponentRecursive<T> {
		node.into_node().get_recursive(world, entity)
	}

	pub fn flatten(&self) -> Vec<Option<&T>> {
		let mut out = self
			.children
			.iter()
			.map(|c| c.flatten())
			.flatten()
			.collect::<Vec<_>>();
		out.push(self.value);
		out
	}
}

impl<'a, T> Display for NodeComponentRecursive<'a, T>
where
	T: IntoNodeComponent + Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let indent =
			std::iter::repeat("  ").take(self.depth).collect::<String>();
		let val = if let Some(val) = self.value {
			format!("{:?}", val)
		} else {
			"None".to_string()
		};
		write!(f, "{indent}{val}")?;
		for child in &self.children {
			write!(f, "\n{child}")?;
		}
		Ok(())
	}
}
