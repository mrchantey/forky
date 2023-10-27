use crate::node::*;
use crate::*;
use bevy_ecs::prelude::*;

#[action(props=Score::Fail)]
pub fn score_always_pass<N: AiNode>(mut query: Query<&mut Prop<Score, N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Pass;
	}
}
#[action(props=Score::Fail)]
pub fn score_always_fail<N: AiNode>(mut query: Query<&mut Prop<Score, N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Fail;
	}
}
