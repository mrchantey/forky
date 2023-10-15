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
	mut query: Query<(
		Entity,
		&mut Prop<NodeState, N>,
		N::ChildQueryOptMut<NodeState>,
	)>,
) {
	for (entity, mut state, children) in query.iter_mut() {
		let mut children = N::children_opt_mut(children);
		if **state == NodeState::Running {
			let next_index =
				children.iter_mut().find_map(|child| match child.get() {
					Some(NodeState::Running) => None,
					Some(NodeState::Success) => {
						child.set(&mut commands, None);
						Some(child.index() + 1)
					}
					Some(NodeState::Failure) => {
						child.set(&mut commands, None);
						**state = NodeState::Failure;
						None
					}
					None => Some(0),
				});
			if let Some(next_index) = next_index {
				if let Some(child) = children.get_mut(next_index) {
					child.set(&mut commands, Some(NodeState::Running));
				} else {
					**state = NodeState::Success;
				}
			}
		} else {
			//TODO this should happen automatically
			commands.entity(entity).remove::<Prop<NodeState, N>>();
			for child in children.iter_mut() {
				child.set(&mut commands, None);
			}
		}
	}
}
