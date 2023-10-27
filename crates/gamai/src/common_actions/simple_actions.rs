use crate::node::*;
use crate::prop::IntoPropBundle;
use crate::*;
use bevy_ecs::prelude::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub struct empty_node;

impl IntoPropBundle for empty_node {
	fn into_bundle<Node: AiNode>(self) -> impl Bundle { () }
}

impl IntoAction for empty_node {
	const IS_EMPTY: bool = true;
	fn into_action_configs<Node: AiNode>(
		self,
	) -> bevy_ecs::schedule::SystemConfigs {
		(|| {}).into_configs()
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
