use crate::*;
use bevy_ecs::prelude::*;


/*
Sequence pseudocode
if my state is not "Running", parent has interrupted so recursively remove node state



*/

#[node_system]
pub fn sequence<N: AiNode>(
	mut commands: Commands,
	mut query: Query<(Entity, &mut ChildNodeState<N>, ChildIter<N>)>,
) {
	for (entity, state, children) in query.iter_mut() {
		let mut children = N::children(children);
		if **state == NodeState::Running {
			let first_child = children.iter().find_map(|child| {
				if let Some(val) = &child.node {
					if ***val == NodeState::Running {
						return Some(***val);
					}
				}
				None
			});
		} else {
			commands.entity(entity).remove::<ChildNodeState<N>>();
			for child in children.iter_mut() {
				child.set_node_state(&mut commands, None);
			}
		}
	}
}
