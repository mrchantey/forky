use crate::*;
use bevy_ecs::prelude::*;


/*
Sequence pseudocode
if my state is not "Running", parent has interrupted so recursively remove node state



*/


#[node_system]
pub fn sequence<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildQuery>,
) {
	// for node in query.iter_mut() {
	// 	let (mut state, mut children) = N::children(node);

	// 	if let Some(node) = &state.node {
	// 		//parent has interrupted
	// 		if ***node == NodeState::Running {
	// 			let first_child = children.iter().find_map(|child| {
	// 				if let Some(val) = &child.node {
	// 					if ***val == NodeState::Running {
	// 						return Some(***val);
	// 					}
	// 				}
	// 				None
	// 			});
	// 		} else {
	// 			state.set_node_state(&mut commands, None);
	// 			for child in children.iter_mut() {
	// 				child.set_node_state(&mut commands, None);
	// 			}
	// 		}
	// 	}
	// }
}
