// pub mod common_solvers {
use crate::*;
use bevy_ecs::prelude::*;


// should probably be a node not a node system
pub fn empty_node() {}

// #[derive(Debug, Default, Clone)]
// #[allow(non_camel_case_types)]
// pub struct empty_node;

// impl IntoNodeSystem<Self> for empty_node {
// 	fn into_node_system<Node: AiNode>(
// 		self,
// 		_schedule: &mut Schedule,
// 		_set: impl SystemSet,
// 	) {
// 	}
// }

#[node_system]
pub fn first_valid_edge<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildQuery, With<DerefNodeState<N>>>,
) {
	for node in query.iter_mut() {
		let mut children = N::children(node);
		for child in children.iter_mut() {
			if **child.edge != EdgeState::Fail {
				// println!("first_valid_edge: setting node state..");
				child.set_node_state(&mut commands, Some(NodeState::Running));
			}
		}
	}
}

#[node_system]
pub fn edge_always_pass<N: AiNode>(mut query: Query<&mut DerefEdgeState<N>>) {
	// println!("edge_always_pass: Running");
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Pass;
	}
}
#[node_system]
pub fn edge_always_fail<N: AiNode>(mut query: Query<&mut DerefEdgeState<N>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Fail;
	}
}
#[node_system]
pub fn node_always_succeed<N: AiNode>(
	mut query: Query<&mut DerefNodeState<N>>,
) {
	for mut node in query.iter_mut() {
		**node = NodeState::Success;
	}
}
#[node_system]
pub fn node_always_fail<N: AiNode>(mut query: Query<&mut DerefNodeState<N>>) {
	for mut node in query.iter_mut() {
		**node = NodeState::Failure;
	}
}
#[node_system]
pub fn print_on_run<N: AiNode>(mut query: Query<&mut DerefNodeState<N>>) {
	// println!("print_on_run: running..");
	for mut node in query.iter_mut() {
		println!("NodeSystem: Running {:?}", **node);
		**node = NodeState::Success;
	}
}

pub fn cleanup_state<N: AiNode>(
	mut commands: Commands,
	query: Query<(Entity, &DerefNodeState<N>), Changed<DerefNodeState<N>>>,
) {
	for (entity, node) in query.iter() {
		if **node != NodeState::Running {
			commands.entity(entity).remove::<DerefNodeState<N>>();
		}
	}
}
pub fn cleanup_child_state<N: AiNode>(
	mut commands: Commands,
	mut query: Query<ChildIter<N>>,
) {
	for children in query.iter_mut() {
		for mut child in N::children(children) {
			if let Some(val) = &child.node && ***val != NodeState::Running {
				child.set_node_state(&mut commands, None);
			}
		}
	}
}
