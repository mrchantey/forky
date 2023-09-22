use crate::*;
use bevy_ecs::prelude::*;

/// Logical `AND`. A node that runs its children in order.
///
/// If a child succeeds it will run the next child.
///
/// If there are no more children it will succeed.
///
/// If a child fails it will fail.
#[node_system]
pub fn sequence<N: AiNode>(
	mut commands: Commands,
	mut query: Query<(Entity, &mut DerefNodeState<N>, ChildIter<N>)>,
) {
	for (entity, mut state, children) in query.iter_mut() {
		let mut children = N::children(children);
		// TODO we shouldnt be checking state, it should be one directional
		if **state == NodeState::Running {
			let next_index = match children.first_with_node_state() {
				Some((child, child_state)) => match child_state {
					NodeState::Running => None,
					NodeState::Success => {
						child.set_node_state(&mut commands, None);
						Some(child.index + 1)
					}
					NodeState::Failure => {
						child.set_node_state(&mut commands, None);
						**state = NodeState::Failure;
						None
					}
				},
				None => Some(0),
			};
			if children
				.try_set_node_state(&mut commands, next_index)
				.is_err()
			{
				**state = NodeState::Success;
			}
		} else {
			commands.entity(entity).remove::<DerefNodeState<N>>();
			for child in children.iter_mut() {
				child.set_node_state(&mut commands, None);
			}
		}
	}
}
