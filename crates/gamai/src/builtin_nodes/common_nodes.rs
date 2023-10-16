// pub mod common_solvers {
use crate::*;
use bevy_ecs::prelude::*;


// should probably be a node not a node system
// pub fn empty_node() {}

//TODO deprecate, dont need it, instead empty attribute
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

#[action]
pub fn score_always_pass<N: AiNode>(mut query: Query<&mut ScoreProp<N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Pass;
	}
}
#[action]
pub fn score_always_fail<N: AiNode>(mut query: Query<&mut ScoreProp<N>>) {
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
			.insert(Prop::<_, N>::new(NodeState::Success));
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
			.insert(Prop::<_, N>::new(NodeState::Failure));
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
			.insert(Prop::<_, N>::new(NodeState::Success));
	}
}


#[action]
pub fn remove_running<N: AiNode>(
	mut commands: Commands,
	added_result: Query<
		Entity,
		(With<Prop<NodeState, N>>, With<Prop<Running, N>>),
	>,
	mut removed_running: RemovedComponents<Prop<Running, N>>, 
	//TODO added_interrupt, recursive cleanup
) {
	// First time around ensure this node doesnt run again
	for entity in added_result.iter() {
		commands.entity(entity).remove::<Prop<Running, N>>();
	}
	// Second time around ensure parent doesnt read state again
	for entity in removed_running.iter() {
		println!("removing");
		commands.entity(entity).remove::<Prop<NodeState, N>>();
	}
}
