use crate::*;
use bevy_ecs::prelude::*;

/// Logical `AND`. A node that runs its children in order.
///
/// If a child succeeds it will run the next child.
///
/// If there are no more children it will succeed.
///
/// If a child fails it will fail.
#[action]
pub fn sequence<N: AiNode>(
	mut commands: Commands,
	mut query: Query<
		(
			Entity,
			N::ChildQueryOptMut<Running>,
			N::ChildQueryOptMut<NodeState>,
		),
		With<Prop<Running, N>>,
	>,
) {
	for (entity, running, out) in query.iter_mut() {
		// if **state == NodeState::Running
		let mut children = std::iter::zip(
			N::children_opt_mut(running).into_iter(),
			N::children_opt_mut(out).into_iter(),
		)
		.collect::<Vec<_>>();

		let next_index = children.iter_mut().find_map(|(running, out)| {
			match (running.get(), out.get()) {
				(_, Some(NodeState::Success)) => {// returned success
					// running.set(&mut commands, None); //should be done in cleanup
					Some(running.index() + 1)
				}
				(_, Some(NodeState::Failure)) => {// returned failure
					// running.set(&mut commands, None); //should be done in cleanup
					commands
						.entity(entity)
						.insert(Prop::<_, N>::new(NodeState::Failure));
					None
				}
				(Some(_), _) => None,//still running
				(None, None) => Some(0),//not running
			}
		});
		if let Some(next_index) = next_index {
			if let Some((running, _)) = children.get_mut(next_index) {
				// println!("setting child");
				running.set(&mut commands, Some(Running));
			} else {
				commands
					.entity(entity)
					.insert(Prop::<_, N>::new(NodeState::Success));
			}
		}
		// } else {
		// 	//TODO this should happen automatically
		// 	commands.entity(entity).remove::<Prop<NodeState, N>>();
		// 	for child in children.iter_mut() {
		// 		child.set(&mut commands, None);
		// 	}
		// }
	}
}
