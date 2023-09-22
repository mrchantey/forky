use crate::*;
use bevy_ecs::prelude::*;

#[node_system]
pub fn fallback<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildQuery>,
) {
	for node in query.iter_mut() {
		let mut children = N::children(node);
		for child in children.iter_mut() {
			// if **child.edge != EdgeState::Fail {
			// 	println!("first_valid_edge: setting node state..");
			// 	child.set_node_state(&mut commands, NodeState::Running);
			// }
		}
		// let a = N::children(node.clone());
		// for child in N::children(node).iter_mut() {
		// if **child.edge != EdgeState::Fail {
		// 	println!("first_valid_edge: setting node state..");
		// 	child.set_node_state(&mut commands, NodeState::Running);
		// }
		// }
	}
}
