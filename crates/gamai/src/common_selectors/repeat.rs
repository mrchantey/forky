use crate::node::*;
use crate::*;
use bevy_ecs::prelude::*;

/// This action will ensure that all children always have the `Running` component
#[action]
pub fn repeat<N: AiNode>(
	mut commands: Commands,
	mut query: Query<
		N::ChildQueryOptMut<Running>,
		With<Prop<Running, N>>,
	>,
) {
	for running in query.iter_mut() {
		let children = N::children_opt_mut(running);
		for mut running in children {
			running.set(&mut commands, Some(Running));
		}
	}
}
