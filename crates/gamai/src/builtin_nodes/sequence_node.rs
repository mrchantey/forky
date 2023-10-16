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
		let mut children = std::iter::zip(
			N::children_opt_mut(running).into_iter(),
			N::children_opt_mut(out).into_iter(),
		)
		.collect::<Vec<_>>();

		// a child is running
		if children.iter().any(|(running, _)| running.get().is_some()) {
			// println!("its still running!");
			continue;
		}

		let next_index = children.iter_mut().find_map(|(running, out)| {
			match (running.get(), out.get()) {
				(_, Some(NodeState::Success)) => Some(running.index() + 1),
				(_, Some(NodeState::Failure)) => None,
				_ => Some(0), //time for first child
			}
		});

		println!("index: {:?}", next_index);
		if let Some(next_index) = next_index {
			if let Some((running, _)) = children.get_mut(next_index) {
				println!("running: {:?}", next_index);
				running.set(&mut commands, Some(Running));
			} else {
				println!("success");
				commands
				.entity(entity)
					.insert(Prop::<_, N>::new(NodeState::Success));
			}
		} else {
			commands
				.entity(entity)
				.insert(Prop::<_, N>::new(NodeState::Failure));
		};
	}
}
