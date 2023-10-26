// use crate::*;
// use bevy_ecs::prelude::*;


// /// Logical OR. A node that runs its children in order until one succeeds.
// ///
// /// If a child fails it will run the next child.
// ///
// /// If there are no more children it will fail.
// ///
// /// If a child succeeds it will succeed.
// #[action]
// pub fn fallback<N: AiNode>(
// 	mut commands: Commands,
// 	mut query: Query<N::ChildQuery>,
// ) {
// 	for node in query.iter_mut() {
// 		let mut children = N::children(node);
// 		for child in children.iter_mut() {
// 			// if **child.edge != EdgeState::Fail {
// 			// 	println!("highest_score: setting node state..");
// 			// 	child.set_node_state(&mut commands, ActionResult::Running);
// 			// }
// 		}
// 		// let a = N::children(node.clone());
// 		// for child in N::children(node).iter_mut() {
// 		// if **child.edge != EdgeState::Fail {
// 		// 	println!("highest_score: setting node state..");
// 		// 	child.set_node_state(&mut commands, ActionResult::Running);
// 		// }
// 		// }
// 	}
// }
