use crate::node::*;
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
			N::ChildQueryOptMut<ActionResult>,
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

		// for (running, out) in children.iter() {
		// 	println!(
		// "index: {:?} running: {:?} out: {:?}",
		// 		running.index(),
		// 		running.get(),
		// 		out.get()
		// 	);
		// }

		let next_index =
			children.iter_mut().fold(Some(0), |prev, (running, out)| {
				match (running.get(), out.get()) {
					(_, Some(ActionResult::Success)) => {
						Some(running.index() + 1)
					}
					(_, Some(ActionResult::Failure)) => None,
					_ => prev,
				}
			});

		if let Some(next_index) = next_index {
			if let Some((running, _)) = children.get_mut(next_index) {
				// println!("running: {:?}", next_index);
				running.set(&mut commands, Some(Running));
			} else {
				// println!("success");
				commands
					.entity(entity)
					.insert(Prop::<_, N>::new(ActionResult::Success));
			}
		} else {
			commands
				.entity(entity)
				.insert(Prop::<_, N>::new(ActionResult::Failure));
		};
	}
}
