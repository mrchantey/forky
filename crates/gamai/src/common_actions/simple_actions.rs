use crate::node::*;
use crate::*;
use bevy_ecs::prelude::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub struct empty_node;

impl IntoAction for empty_node {
	const IS_EMPTY: bool = true;
	fn into_action_configs<Node: AiNode>(
		self,
	) -> bevy_ecs::schedule::SystemConfigs {
		(|| {}).into_configs()
	}
}

#[action]
pub fn score_always_pass<N: AiNode>(mut query: Query<&mut Prop<Score, N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Pass;
	}
}
#[action]
pub fn score_always_fail<N: AiNode>(mut query: Query<&mut Prop<Score, N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Fail;
	}
}
#[action]
pub fn node_always_succeed<N: AiNode>(
	mut commands: Commands,
	mut query: Query<Entity, With<Prop<Running, N>>>,
) {
	for entity in query.iter_mut() {
		commands
			.entity(entity)
			.insert(Prop::<_, N>::new(ActionResult::Success));
	}
}
#[action]
pub fn node_always_fail<N: AiNode>(
	mut commands: Commands,
	mut query: Query<Entity, With<Prop<Running, N>>>,
) {
	for entity in query.iter_mut() {
		commands
			.entity(entity)
			.insert(Prop::<_, N>::new(ActionResult::Failure));
	}
}

#[action]
pub fn always_succeed_and_print<N: AiNode>(
	mut commands: Commands,
	mut query: Query<Entity, With<Prop<Running, N>>>,
) {
	for entity in query.iter_mut() {
		println!("Node running: {}", N::DEPTH);
		commands
			.entity(entity)
			.insert(Prop::<_, N>::new(ActionResult::Success));
	}
}
