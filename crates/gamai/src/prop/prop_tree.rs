use super::*;
use crate::node::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

/// Get all values for a given prop in a tree.
#[derive(Debug, Clone)]
pub struct PropTree<'a, T: IntoProp> {
	pub depth: usize,
	pub value: Option<&'a T>,
	pub children: Vec<PropTree<'a, T>>,
}

impl<'a, T: IntoProp> PropTree<'a, T> {
	pub fn new<M, N: AiNode>(
		node: impl IntoNode<M, Out = N>,
		world: &World,
		entity: Entity,
	) -> PropTree<T> {
		node.into_node().get_recursive(world, entity)
	}


	/// Get all values, in a depth-first order.
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

impl<'a, T> Display for PropTree<'a, T>
where
	T: IntoProp + Debug,
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
