use crate::*;
use bevy_ecs::prelude::*;
use std::ops::Deref;

/// Tuple used in queries to access child states:
///
/// `(&mut Entity,(&mut Child1,(&mut Child2)))`
pub type ChildIter<T, N> = <N as AiNode>::ChildQuery<T>;
pub type ChildIterOptional<T, N> = <N as AiNode>::ChildQueryOpt<T>;

// #[derive(Clone)]
pub struct ChildState<'a, T: IntoNodeComponent> {
	pub index: usize,
	pub entity: Entity,
	pub value: &'a dyn Deref<Target = T>,
}
pub struct ChildStateOpt<'a, T: IntoNodeComponent> {
	pub index: usize,
	pub entity: Entity,
	pub value: Option<&'a dyn Deref<Target = T>>,
}
pub struct ChildStateMut<'a, T: IntoNodeComponent> {
	pub index: usize,
	pub entity: Entity,
	pub value: Mut<'a, T>,
}
pub struct ChildStateOptMut<'a, T: IntoNodeComponent> {
	pub index: usize,
	pub entity: Entity,
	pub value: Option<Mut<'a, T>>,
}


impl<'a, T: IntoNodeComponent> ChildState<'a, T> {
	/// helper function for setting node state from a context where the concrete type is not known.
	///
	/// if current and next are `None` this is a noop
	///
	/// if current and next are `Some` state will be mutated instead of command created.
	pub fn set_node_state(
		&mut self,
		_commands: &mut Commands,
		_state: Option<NodeState>,
	) {
		panic!("deleteme");
	}
}


pub trait ChildQueryExt {
	type Out;
	fn to_props(self) -> Self::Out;
}

pub trait ChildVecTrait<'a, T: IntoNodeComponent> {
	// fn first_with_state(&mut self) -> Option<&mut ChildState<'a, T>>;
	// Sets the node state of the child at that index,
	// fn try_set_state(
	// 	&mut self,
	// 	commands: &mut Commands,
	// 	index: Option<usize>,
	// 	state: Option<NodeState>,
	// ) -> anyhow::Result<()>;
}

// impl<'a> ChildVecTrait<'a> for Vec<ChildState<'a>> {
// 	fn first_with_node_state(
// 		&mut self,
// 	) -> Option<(&mut ChildState<'a>, NodeState)> {
// 		self.iter_mut().find_map(|child| {
// 			if let Some(val) = child.node.as_ref().map(|val| ***val) {
// 				Some((child, val))
// 			} else {
// 				None
// 			}
// 		})
// 	}

// 	fn try_set_node_state(
// 		&mut self,
// 		commands: &mut Commands,
// 		index: Option<usize>,
// 		state: Option<NodeState>,
// 	) -> anyhow::Result<()> {
// 		if let Some(index) = index && let Some(child) = self.get_mut(index) {
// 			child.set_node_state(commands, state);
// 			Ok(())
// 		} else {
// 			anyhow::bail!("index out of bounds")
// 		}
// 	}
// }
