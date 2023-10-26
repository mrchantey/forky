use crate::node::*;
use crate::prop::*;
use crate::*;
use bevy_ecs::prelude::*;

///Runs the child with the highest [`Score`]
/// ## Child Props
/// - [`Running`]
/// - [`Score`]
#[action]
pub fn highest_score<N: AiNode>(
	mut commands: Commands,
	mut query: Query<
		(
			Entity,
			N::ChildQuery<Score>,
			N::ChildQueryOptMut<Running>,
			N::ChildQueryOpt<ActionResult>,
		),
		With<Prop<Running, N>>,
	>,
) {
	'per_entity: for (entity, scores, states, results) in query.iter_mut() {
		let mut highest = None;
		let mut highest_state = None;

		for ((score, state), result) in N::children(scores)
			.into_iter()
			.zip(N::children_opt_mut(states).into_iter())
			.zip(N::children_opt(results).into_iter())
			.collect::<Vec<_>>()
		{
			let score = **score.get();
			if let Some(_) = state.get() {
				// a child is running, continue
				continue 'per_entity;
			} else if let Some(result) = result.get() {
				// a child has completed
				commands
					.entity(entity)
					.insert(Prop::<ActionResult, N>::new(**result));
			} else if score != Score::Fail {
				if let Some(highest) = highest && score < highest{
					continue;
				}
				// a child has the highest score
				highest = Some(score);
				highest_state = Some(state);
			}
		}

		if let Some(mut highest) = highest_state {
			highest.set(&mut commands, Some(Running));
		}
	}
}
