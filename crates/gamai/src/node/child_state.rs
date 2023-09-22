use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;


/// Tuple used in queries to access child states: `(Entity,(Child1,(Child2)))`
pub type ChildIter<N> = <N as AiNode>::ChildQuery;

// #[derive(Clone)]
pub struct ChildState<'a> {
	pub index: usize,
	pub entity: Entity,
	pub edge: DerefEdge<'a>,
	pub node: Option<DerefNode<'a>>,

	//this is horrid, we need to use generics and traits instead
	pub set_node_state_func: fn(&mut Commands, Entity, state: NodeState),
	pub remove_node_state_func: fn(&mut Commands, Entity),
}


impl<'a> ChildState<'a> {
	/// helper function for setting node state from a context where the concrete type is not known.
	///
	/// if current and next are `None` this is a noop
	///
	/// if current and next are `Some` state will be mutated instead of command created.
	pub fn set_node_state(
		&mut self,
		commands: &mut Commands,
		state: Option<NodeState>,
	) {
		match (&mut self.node, state) {
			(None, None) => {
				//noop
			}
			(None, Some(next)) => {
				(self.set_node_state_func)(commands, self.entity, next);
			}
			(Some(_), None) => {
				(self.remove_node_state_func)(commands, self.entity);
			}
			(Some(current), Some(next)) => {
				if ***current != next {
					***current = next;
				}
			}
		}
	}
}

// pub type DerefEdge<'a> = Mut<'_, gamai::DerefNodeState<Child0>>;
// pub type DerefNode<'a> = Mut<'_, gamai::DerefNodeState<Child0>>;
pub trait ChildVecTrait<'a> {
	fn first_with_node_state(
		&mut self,
	) -> Option<(&mut ChildState<'a>, NodeState)>;
	fn try_set_node_state(
		&mut self,
		commands: &mut Commands,
		index: Option<usize>,
	) -> anyhow::Result<()>;
}

impl<'a> ChildVecTrait<'a> for Vec<ChildState<'a>> {
	fn first_with_node_state(
		&mut self,
	) -> Option<(&mut ChildState<'a>, NodeState)> {
		self.iter_mut().find_map(|child| {
			if let Some(val) = child.node.as_ref().map(|val| ***val) {
				Some((child, val))
			} else {
				None
			}
		})
	}

	fn try_set_node_state(
		&mut self,
		commands: &mut Commands,
		index: Option<usize>,
	) -> anyhow::Result<()> {
		if let Some(index) = index && let Some(child) = self.get_mut(index) {
			child.set_node_state(commands, Some(NodeState::Running));
			Ok(())
		} else {
			anyhow::bail!("index out of bounds")
		}
	}
}

// impl<N> ChildIterTrait for ChildIter<N> {
// 	fn children<'a>(
// 		item: <Self::ChildQuery as WorldQuery>::Item<'a>,
// 	) -> Vec<ChildState<'a>> {
// 		<Self as AiNode>::children(item)
// 	}
// }
