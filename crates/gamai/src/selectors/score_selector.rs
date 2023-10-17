use crate::*;
use crate::node::*;
use bevy_ecs::prelude::*;

#[action]
pub fn first_passing_score<N: AiNode>(
	mut commands: Commands,
	mut query: Query<
		(N::ChildQuery<Score>, N::ChildQueryOptMut<Running>),
		With<Prop<Running, N>>,
	>,
) {
	for (scores, states) in query.iter_mut() {
		for (score, mut state) in std::iter::zip(
			N::children(scores).into_iter(),
			N::children_opt_mut(states).into_iter(),
		)
		.collect::<Vec<_>>()
		{
			if **score.get() != Score::Fail {
				state.set(&mut commands, Some(Running));
			}
		}
	}
}
